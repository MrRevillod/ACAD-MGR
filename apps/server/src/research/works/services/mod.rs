mod openalex;

use crate::research::*;
use crate::shared::AppResult;

pub use openalex::*;

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
			Some(sid) => self.sources.find_by_id(&sid).await?,
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
		let orcid = self
			.works
			.academic_has_orcid(academic_id)
			.await?
			.ok_or(WorksError::AcademicNotFound)?;

		let oa_works = self.openalex.list_works_by_orcid(&orcid).await?;
		let works_fetched = oa_works.len();

		let mut created = 0usize;
		let mut skipped = 0usize;
		let mut authorships_count = 0usize;
		let mut topics_count = 0usize;
		let mut keywords_count = 0usize;
		let mut errors = Vec::new();

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

		for oa_work in &oa_works {
			let work_result = process_single_work(
				&self.works,
				&self.sources,
				&self.authorships,
				&self.classification,
				oa_work,
				unknown_topic_id,
				unknown_keyword_id,
			)
			.await;

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
}

struct ProcessStats {
	was_inserted: bool,
	authorships: usize,
	topics: usize,
	keywords: usize,
}

async fn process_single_work(
	works_repo: &WorksRepository,
	sources_repo: &SourcesRepository,
	authorships_repo: &AuthorshipsRepository,
	classification_repo: &WorkClassificationRepository,
	oa_work: &papers_openalex::Work,
	unknown_topic_id: Option<crate::research::classification::ResearchTopicId>,
	unknown_keyword_id: Option<crate::research::classification::ResearchKeywordId>,
) -> AppResult<ProcessStats> {
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
		Some(
			sources_repo
				.save(&Source {
					id: SourceId::new(),
					openalex_id: s.id.clone().unwrap_or_default(),
					display_name: s.display_name.clone().unwrap_or_default(),
					ty: source_ty,
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
	let inserted = works_repo.insert_work(&new_work).await?;

	let work_id = match inserted {
		Some(id) => id,
		None => {
			return Ok(ProcessStats {
				was_inserted: false,
				authorships: 0,
				topics: 0,
				keywords: 0,
			});
		}
	};

	let mut authorships = 0usize;
	if let Some(auths) = &oa_work.authorships {
		for auth in auths {
			let orcid = match auth.author.as_ref().and_then(|a| a.orcid.as_deref()) {
				Some(o) => o,
				None => continue,
			};

			let academic_name = works_repo.find_academic_name_by_orcid(orcid).await?;
			let is_external = academic_name.is_none();
			let name = if is_external {
				auth.author
					.as_ref()
					.and_then(|a| a.display_name.as_deref())
					.unwrap_or("Unknown")
					.to_string()
			} else {
				academic_name.unwrap_or_default()
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
				affiliations: auth.raw_affiliation_strings.clone().unwrap_or_default(),
				position,
			};
			authorships_repo.insert(&new_authorship).await?;
			authorships += 1;
		}
	}

	let mut topics = 0usize;
	if let Some(work_topics) = &oa_work.topics {
		for t in work_topics {
			let topic_id = match &t.id {
				Some(id) => classification_repo
					.find_topic_by_openalex_id(id)
					.await?
					.map(|rt| *rt.id)
					.or(unknown_topic_id.map(|id| *id)),
				None => unknown_topic_id.map(|id| *id),
			};

			if let Some(tid) = topic_id {
				works_repo
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
					Some(classification_repo.upsert_keyword(id, name).await?)
				}
				None => unknown_keyword_id,
			};

			if let Some(kid) = keyword_id {
				let score = k.score.unwrap_or(0.0);
				works_repo.link_keyword(&work_id, *kid, score).await?;
				kw_count += 1;
			}
		}
	}

	Ok(ProcessStats {
		was_inserted: true,
		authorships,
		topics,
		keywords: kw_count,
	})
}
