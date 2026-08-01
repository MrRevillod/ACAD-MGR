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
}

impl WorksService {
	pub async fn list(&self, query: &GetWorksQuery) -> AppResult<Vec<WorkView>> {
		let mut views = self.works.list(query).await?;

		for view in &mut views {
			view.work = view.work.resolve();
		}

		Ok(views)
	}

	pub async fn find_by_id(&self, id: WorkId) -> AppResult<WorkView> {
		let Some(mut view) = self.works.find_by_id(&id).await? else {
			return Err(WorksError::NotFound)?;
		};

		view.work = view.work.resolve();

		let work_id = view.work.id;

		view.source = match view.work.source_id {
			Some(source_id) => self.sources.find_source_view_by_id(&source_id).await?,
			None => None,
		};

		view.authorships = Some(self.authorships.list(&work_id).await?);
		view.topics = Some(self.works.list_topics_by_work(&work_id).await?);
		view.keywords = Some(self.works.list_keywords_by_work(&work_id).await?);

		Ok(view)
	}

	pub async fn update_overrides(
		&self,
		work_id: WorkId,
		input: WorkOverridesInput,
	) -> AppResult<()> {
		let Some(mut work) = self.works.find_work(&work_id).await? else {
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
		let Some(mut work) = self.works.find_work(&work_id).await? else {
			return Err(WorksError::NotFound)?;
		};

		work.overrides = WorkOverrides::default();
		work.updated_at = chrono::Utc::now();
		self.works.save(&work).await
	}
}
