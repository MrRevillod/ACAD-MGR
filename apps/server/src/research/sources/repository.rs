use crate::research::*;
use crate::shared::{AppResult, Database};

use sqlx::Row;
use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct SourcesRepository {
	database: Arc<Database>,
}

impl SourcesRepository {
	pub async fn find_by_id(&self, id: &SourceId) -> AppResult<Option<Source>> {
		sqlx::query_as::<_, Source>(
			"SELECT id, openalex_id, display_name, ty FROM sources WHERE id = $1",
		)
		.bind(id)
		.fetch_optional(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn save(&self, source: &Source) -> AppResult<SourceId> {
		let row = sqlx::query(
			"INSERT INTO sources (openalex_id, display_name, ty)
            VALUES ($1, $2, $3)
            ON CONFLICT (openalex_id) DO UPDATE SET
            display_name = EXCLUDED.display_name,
            ty = EXCLUDED.ty RETURNING id",
		)
		.bind(&source.openalex_id)
		.bind(&source.display_name)
		.bind(&source.ty)
		.fetch_one(self.database.pool())
		.await?;

		Ok(SourceId::from_uuid(row.get("id")))
	}
}
