mod import;
mod openalex;

use crate::research::*;
use crate::shared::AppResult;

pub use import::WorksImportService;
pub use openalex::*;

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct WorksService {
	works: Arc<WorksRepository>,
	sources: Arc<SourcesRepository>,
	authorships: Arc<AuthorshipsRepository>,
	classification: Arc<WorkClassificationRepository>,
}

impl WorksService {
	pub async fn list(&self, query: &GetWorksQuery) -> AppResult<Vec<WorkView>> {
		let works = self.works.list(query).await?;
		self.to_work_views(works, false).await
	}

	pub async fn find_by_id(&self, id: WorkId) -> AppResult<WorkView> {
		let Some(work) = self.works.find_by_id(&id).await? else {
			return Err(WorksError::NotFound)?;
		};

		let mut views = self.to_work_views(vec![work], true).await?;
		Ok(views.pop().unwrap())
	}

	pub async fn update_overrides(
		&self,
		work_id: WorkId,
		input: WorkOverridesInput,
	) -> AppResult<()> {
		let Some(mut work) = self.works.find_by_id(&work_id).await? else {
			return Err(WorksError::NotFound)?;
		};

		if let Some(v) = input.title {
			work.overrides.title = v;
		}
		if let Some(v) = input.abstract_text {
			work.overrides.abstract_text = v;
		}
		if let Some(v) = input.doi {
			work.overrides.doi = v;
		}
		if let Some(v) = input.publication_year {
			work.overrides.publication_year = v;
		}
		if let Some(v) = input.is_accepted {
			work.overrides.is_accepted = v;
		}
		if let Some(v) = input.is_published {
			work.overrides.is_published = v;
		}
		if let Some(v) = input.research_line_id {
			work.overrides.research_line_id = v;
		}

		work.updated_at = chrono::Utc::now();
		self.works.save(&work).await
	}

	pub async fn clear_overrides(&self, work_id: WorkId) -> AppResult<()> {
		let Some(mut work) = self.works.find_by_id(&work_id).await? else {
			return Err(WorksError::NotFound)?;
		};

		work.overrides = WorkOverrides::default();
		work.updated_at = chrono::Utc::now();
		self.works.save(&work).await
	}

	async fn to_work_views(&self, works: Vec<Work>, detail: bool) -> AppResult<Vec<WorkView>> {
		if works.is_empty() {
			return Ok(Vec::new());
		}

		let source_ids: Vec<SourceId> = works.iter().filter_map(|w| w.source_id).collect();
		let kinds = self.sources.find_kinds_by_source_ids(&source_ids).await?;

		let work_ids: Vec<WorkId> = works.iter().map(|w| w.id).collect();
		let override_lines: Vec<(WorkId, Option<uuid::Uuid>)> = works
			.iter()
			.map(|w| (w.id, w.overrides.research_line_id))
			.collect();

		let lines = self
			.classification
			.resolve_research_lines_for_works(&work_ids, &override_lines)
			.await?;

		let mut views = Vec::with_capacity(works.len());

		for work in works {
			let overridden_fields = work.overrides.overridden_field_names();
			let journal_kind = work
				.source_id
				.and_then(|sid| kinds.get(&sid).copied().flatten());
			let line = lines.get(&work.id);
			let research_line_id = line.map(|l| l.id);
			let research_line_name = line.map(|l| l.name.clone());
			let id = work.id;

			let (source, authorships, topics, keywords) = if detail {
				let source = match work.source_id {
					Some(sid) => self.sources.find_source_view_by_id(&sid).await?,
					None => None,
				};
				let authorships = self.authorships.list(&id).await?;
				let topics = self.classification.list_topics_by_work(&id).await?;
				let keywords = self.classification.list_keywords_by_work(&id).await?;
				(
					source,
					Some(authorships),
					Some(topics),
					Some(keywords),
				)
			} else {
				(None, None, None, None)
			};

			views.push(WorkView {
				work: work.resolve(),
				overridden_fields,
				journal_kind,
				research_line_id,
				research_line_name,
				source,
				authorships,
				topics,
				keywords,
			});
		}

		Ok(views)
	}
}
