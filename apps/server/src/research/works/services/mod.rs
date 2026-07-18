mod openalex;

use crate::academic::AcademicsRepository;
use crate::research::*;
use crate::shared::AppResult;

pub use openalex::*;

use html_escape::decode_html_entities;
use papers_openalex::Work as OpenAlexWork;
use std::str::FromStr;
use std::sync::Arc;
use sword::prelude::*;
use uuid::Uuid;

#[injectable]
pub struct WorksService {
	works: Arc<WorksRepository>,
	sources: Arc<SourcesRepository>,
	authorships: Arc<AuthorshipsRepository>,
	classification: Arc<WorkClassificationRepository>,
	academics: Arc<AcademicsRepository>,
	openalex: Arc<OpenAlexClient>,
}

impl WorksService {
	pub async fn list(&self, query: &GetWorksQuery) -> AppResult<Vec<Work>> {
		self.works.list(query).await
	}

	pub async fn find_by_id(&self, id: WorkId) -> AppResult<WorkDetailView> {
		let Some(work) = self.works.find_by_id(&id).await? else {
			return Err(WorksError::NotFound)?;
		};

		let source = match work.primary_source_id {
			Some(sid) => self.sources.find_source_view_by_id(&sid).await?,
			None => None,
		};

		let authorships = self.authorships.list(&work.id).await?;
		let topics = self.classification.list_topics_by_work(&work.id).await?;
		let keywords = self.classification.list_keywords_by_work(&work.id).await?;

		Ok(WorkDetailView {
			work,
			source,
			authorships,
			topics,
			keywords,
		})
	}

	pub async fn sync_from_openalex(&self, academic_id: Uuid) -> AppResult<SyncResultView> {
		let academic = self
			.academics
			.find_by_id(&crate::academic::AcademicId::from_uuid(academic_id))
			.await?
			.ok_or(WorksError::AcademicNotFound)?;

		let orcid = academic.orcid.ok_or(WorksError::AcademicWithoutOrcid)?;

		let oa_works = self.openalex.list_works_by_orcid(&orcid).await?;
		let works_fetched = oa_works.len();

		let mut created = 0usize;
		let mut skipped = 0usize;
		let mut authorships_count = 0usize;
		let mut topics_count = 0usize;
		let mut keywords_count = 0usize;
		let mut errors = Vec::new();

		for oa_work in &oa_works {
			if oa_work.r#type != Some("article".to_string()) {
				skipped += 1;
				continue;
			}

			let work_result = self.process_single_work(oa_work).await;

			match work_result {
				Ok(stats) => {
					if stats.was_inserted {
						created += 1;
						authorships_count += stats.authorships;
						topics_count += stats.topics;
						keywords_count += stats.keywords;
					} else {
						skipped += 1;
					}
				}
				Err(e) => {
					errors.push(format!("{}: {}", oa_work.id, e));
				}
			}
		}

		Ok(SyncResultView {
			academic_id,
			academic_orcid: orcid,
			works_fetched,
			works_created: created,
			works_skipped: skipped,
			authorships_inserted: authorships_count,
			topics_linked: topics_count,
			keywords_linked: keywords_count,
			errors,
		})
	}

	async fn process_single_work(
		&self,
		oa_work: &OpenAlexWork,
	) -> AppResult<WorkImportProcessStats> {
		let title = oa_work
			.title
			.clone()
			.or_else(|| oa_work.display_name.clone())
			.unwrap_or_default();

		let pub_date = oa_work
			.publication_date
			.as_deref()
			.and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());

		let work_type = oa_work
			.r#type
			.as_deref()
			.and_then(|s| WorkType::from_str(s).ok())
			.unwrap_or(WorkType::Other);

		let lang = oa_work.language.clone().unwrap_or_else(|| "en".to_string());

		let (is_accepted, is_published) = oa_work
			.primary_location
			.as_ref()
			.map(|loc| {
				(
					loc.is_accepted.unwrap_or(false),
					loc.is_published.unwrap_or(false),
				)
			})
			.unwrap_or((false, false));

		let source_id = if let Some(s) = oa_work
			.primary_location
			.as_ref()
			.and_then(|l| l.source.as_ref())
		{
			let source_ty = s.r#type.clone().unwrap_or_else(|| "unknown".to_string());
			let issn_l = s.issn_l.as_deref().and_then(Source::normalize_issn);
			let issn: Option<Vec<String>> = s.issn.as_ref().and_then(|vec| {
				let normalized: Vec<String> = vec
					.iter()
					.filter_map(|v| Source::normalize_issn(v))
					.collect();
				if normalized.is_empty() {
					None
				} else {
					Some(normalized)
				}
			});
			Some(
				self.sources
					.save(&Source {
						id: SourceId::new(),
						openalex_id: s.id.clone().unwrap_or_default(),
						display_name: s.display_name.clone().unwrap_or_default(),
						ty: source_ty,
						issn_l,
						issn,
					})
					.await?,
			)
		} else {
			None
		};

		let new_work = NewWork {
			openalex_id: oa_work.id.clone(),
			title,
			abstract_text: oa_work.abstract_text.clone(),
			doi: oa_work.doi.clone(),
			publication_date: pub_date,
			publication_year: oa_work.publication_year.map(|y| y as i16),
			ty: work_type,
			lang,
			is_accepted,
			is_published,
			primary_source_id: source_id,
		};
		let inserted = self.works.insert_work(&new_work).await?;

		let work_id = match inserted {
			Some(id) => id,
			None => {
				return Ok(WorkImportProcessStats {
					was_inserted: false,
					authorships: 0,
					topics: 0,
					keywords: 0,
				});
			}
		};

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
		if let Some(auths) = &oa_work.authorships {
			for auth in auths {
				let orcid = match auth.author.as_ref().and_then(|a| a.orcid.as_deref()) {
					Some(o) => o,
					None => continue,
				};

				let (is_external, name) = match self.academics.find_by_orcid(orcid).await? {
					Some(a) => (false, a.full_name()),
					None => {
						let display = auth
							.author
							.as_ref()
							.and_then(|a| a.display_name.as_deref())
							.unwrap_or("Unknown")
							.to_string();
						(true, display)
					}
				};

				let position = match auth.author_position.as_deref() {
					Some("first") => AuthorshipPosition::First,
					Some("last") => AuthorshipPosition::Last,
					_ => AuthorshipPosition::Middle,
				};

				let new_authorship = NewAuthorship {
					work_id,
					orcid: orcid.to_string(),
					name,
					is_external,
					is_corresponding: auth.is_corresponding.unwrap_or(false),
					affiliations: auth
						.raw_affiliation_strings
						.clone()
						.unwrap_or_default()
						.into_iter()
						.map(|s| decode_html_entities(&s).to_string())
						.collect(),
					position,
				};
				self.authorships.insert(&new_authorship).await?;
				authorships += 1;
			}
		}

		let mut topics = 0usize;
		if let Some(work_topics) = &oa_work.topics {
			for t in work_topics {
				let topic_id = match &t.id {
					Some(id) => self
						.classification
						.find_topic_by_openalex_id(id)
						.await?
						.map(|rt| *rt.id)
						.or(unknown_topic_id.map(|id| *id)),
					None => unknown_topic_id.map(|id| *id),
				};

				if let Some(tid) = topic_id {
					self.works
						.link_topic(&work_id, tid, t.score.unwrap_or(0.0))
						.await?;
					topics += 1;
				}
			}
		}

		let mut kw_count = 0usize;
		if let Some(kws) = &oa_work.keywords {
			for k in kws {
				let keyword_id = match &k.id {
					Some(id) => {
						let name = k.display_name.as_deref().unwrap_or("Unknown");
						Some(self.classification.upsert_keyword(id, name).await?)
					}
					None => unknown_keyword_id,
				};

				if let Some(kid) = keyword_id {
					let score = k.score.unwrap_or(0.0);
					self.works.link_keyword(&work_id, *kid, score).await?;
					kw_count += 1;
				}
			}
		}

		Ok(WorkImportProcessStats {
			was_inserted: true,
			authorships,
			topics,
			keywords: kw_count,
		})
	}
}
