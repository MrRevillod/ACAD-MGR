use crate::research::*;
use crate::shared::{AppError, AppResult, Database};

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct SourcesRepository {
	database: Arc<Database>,
}

impl SourcesRepository {
	pub async fn find_by_id(&self, id: &SourceId) -> AppResult<Option<Source>> {
		Source::filter_by_id(id)
			.exec(&mut self.database.pool())
			.await?
			.map_err(AppError::from)
	}

	pub async fn save(&self, source: &Source) -> AppResult<()> {
		Source::upsert_by_id(source.id)
			.openalex_id(&source.openalex_id)
			.display_name(&source.display_name)
			.ty(&source.ty)
			.issn(source.issn.clone())
			.exec(&mut self.database.pool())
			.await?
			.map_err(AppError::from)?;

		if let Some(ref issns) = source.issn {
			toasty::sql::statement(
				"UPDATE sources SET journal_issn_id = (
					SELECT id FROM journal_issn WHERE issn = ANY($1) LIMIT 1
				) WHERE id = $2",
			)
			.bind(issns)
			.bind(&source.id)
			.exec(&mut self.database.pool())
			.await?;
		}

		Ok(())
	}
}
