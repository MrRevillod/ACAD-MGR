use crate::academic::{AcademicId, AcademicListFilter, AcademicsRepository};
use crate::research::*;
use crate::shared::AppResult;

use papers_openalex::Work as OpenAlexWork;
use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct WorksImportService {
	works: Arc<WorksRepository>,
	sources: Arc<SourcesRepository>,
	authorships: Arc<AuthorshipsRepository>,
	classification: Arc<WorkClassificationRepository>,
	academics: Arc<AcademicsRepository>,
	openalex: Arc<OpenAlexClient>,
}

impl WorksImportService {
	pub async fn sync_from_openalex(&self, academic_id: AcademicId) -> AppResult<SyncResultView> {
		let academic = self
			.academics
			.find_by_id(&academic_id)
			.await?
			.ok_or(WorksError::AcademicNotFound)?;

		let orcid = academic.orcid.ok_or(WorksError::AcademicWithoutOrcid)?;

		let oa_works = self.openalex.list_all_works_by_orcid(&orcid).await?;
		let works_fetched = oa_works.len();

		let mut created = 0usize;
		let mut authorships_count = 0usize;
		let mut topics_count = 0usize;
		let mut keywords_count = 0usize;
		let mut errors = Vec::new();

		for oa_work in &oa_works {
			if oa_work.ty() != WorkType::Article {
				continue;
			}

			let work_result = self.process_single_work(oa_work).await;

			match work_result {
				Ok(stats) if stats.was_inserted => {
					created += 1;
					authorships_count += stats.authorships;
					topics_count += stats.topics;
					keywords_count += stats.keywords;
				}
				Ok(_) => {}
				Err(e) => {
					errors.push(format!("{}: {e}", oa_work.id));
				}
			}
		}

		Ok(SyncResultView {
			academic_id,
			works_fetched,
			works_created: created,
			authorships_inserted: authorships_count,
			topics_linked: topics_count,
			keywords_linked: keywords_count,
			errors,
		})
	}

	pub async fn sync_all_academics(&self) -> AppResult<Vec<SyncResultView>> {
		let academics = self.academics.list(AcademicListFilter::default()).await?;
		let mut results = Vec::with_capacity(academics.len());

		for academic in &academics {
			match self.sync_from_openalex(academic.id).await {
				Ok(result) => results.push(result),
				Err(e) => {
					results.push(SyncResultView {
						academic_id: academic.id,
						works_fetched: 0,
						works_created: 0,
						authorships_inserted: 0,
						topics_linked: 0,
						keywords_linked: 0,
						errors: vec![e.to_string()],
					});
				}
			}
		}

		Ok(results)
	}

	async fn process_single_work(
		&self,
		oa_work: &OpenAlexWork,
	) -> AppResult<WorkImportProcessStats> {
		let source_issns = oa_work.source_issns();
		let source_id = self.ensure_source(oa_work.source(), source_issns).await?;

		let (work, was_inserted) = match self.works.find_by_openalex_id(&oa_work.id).await? {
			Some(mut work) => {
				oa_work.apply_to_work(&mut work, source_id);
				self.works.save(&work).await?;
				(work, false)
			}
			None => {
				let work = oa_work.to_work(source_id);
				self.works.save(&work).await?;
				(work, true)
			}
		};

		if !was_inserted {
			return Ok(WorkImportProcessStats {
				was_inserted: false,
				authorships: 0,
				topics: 0,
				keywords: 0,
			});
		}

		let unknown_topic_id = self
			.classification
			.unknown_topic_id()
			.await
			.ok()
			.flatten()
			.map(|t| t.id);

		let unknown_keyword_id = self
			.classification
			.unknown_keyword_id()
			.await
			.ok()
			.flatten()
			.map(|k| k.id);

		let mut authorships = 0usize;
		for data in oa_work.authorships() {
			let (is_external, name) = match self.academics.find_by_orcid(&data.orcid).await? {
				Some(a) => (false, a.full_name()),
				None => (true, data.display_name),
			};

			let authorship = Authorship::builder()
				.work_id(work.id)
				.orcid(data.orcid)
				.name(name)
				.is_external(is_external)
				.is_corresponding(data.is_corresponding)
				.affiliations(data.affiliations)
				.position(data.position)
				.maybe_academic_id(None)
				.build();

			self.authorships.save(&authorship).await?;
			authorships += 1;
		}

		let mut topics = 0usize;
		for t in oa_work.topic_refs() {
			let topic_id = match t.openalex_id.as_deref() {
				Some(id) => self
					.classification
					.find_topic_by_openalex_id(id)
					.await?
					.map(|rt| *rt.id)
					.or(unknown_topic_id.map(|id| *id)),
				None => unknown_topic_id.map(|id| *id),
			};

			if let Some(tid) = topic_id {
				let score = WorkTopicScore::builder()
					.work_id(work.id)
					.topic_id(tid)
					.score(t.score)
					.build();
				self.works.save_topic_score(&score).await?;
				topics += 1;
			}
		}

		let mut kw_count = 0usize;
		for k in oa_work.keyword_refs() {
			let keyword_id = match k.openalex_id.as_deref() {
				Some(id) => Some(self.ensure_keyword(id, &k.name).await?),
				None => unknown_keyword_id,
			};

			if let Some(kid) = keyword_id {
				let score = WorkKeywordScore::builder()
					.work_id(work.id)
					.keyword_id(*kid)
					.score(k.score)
					.build();
				self.works.save_keyword_score(&score).await?;
				kw_count += 1;
			}
		}

		Ok(WorkImportProcessStats {
			was_inserted: true,
			authorships,
			topics,
			keywords: kw_count,
		})
	}

	async fn ensure_source(
		&self,
		source: Option<Source>,
		issns: Option<Vec<String>>,
	) -> AppResult<Option<SourceId>> {
		let Some(mut incoming) = source else {
			return Ok(None);
		};

		incoming.issn = match issns {
			Some(issns) => self.sources.resolve_issn(&issns).await?,
			None => None,
		};

		match self
			.sources
			.find_by_openalex_id(&incoming.openalex_id)
			.await?
		{
			Some(mut existing) => {
				existing.name = incoming.name;
				existing.ty = incoming.ty;
				existing.issn = incoming.issn;
				self.sources.save(&existing).await?;
				Ok(Some(existing.id))
			}
			None => {
				self.sources.save(&incoming).await?;
				Ok(Some(incoming.id))
			}
		}
	}

	async fn ensure_keyword(&self, openalex_id: &str, name: &str) -> AppResult<KeywordId> {
		match self
			.classification
			.find_keyword_by_openalex_id(openalex_id)
			.await?
		{
			Some(mut existing) => {
				existing.name = name.to_string();
				self.classification.save_keyword(&existing).await?;
				Ok(existing.id)
			}
			None => {
				let keyword = Keyword::builder()
					.openalex_id(openalex_id.to_string())
					.name(name.to_string())
					.build();
				self.classification.save_keyword(&keyword).await?;
				Ok(keyword.id)
			}
		}
	}
}
