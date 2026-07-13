use crate::research::sources::source_view::SourceView;
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
	pub async fn find_source_view_by_id(&self, id: &SourceId) -> AppResult<Option<SourceView>> {
		sqlx::query_as::<_, SourceView>(
			r#"SELECT s.id, s.openalex_id, s.display_name, s.ty, s.issn_l, s.issn,
			          COALESCE(
			              ARRAY(SELECT DISTINCT unnest(ji.kinds)
			                    FROM journal_issn ji
			                    WHERE ji.issn = s.issn_l
			                       OR ji.eissn = s.issn_l
			                       OR ji.issn = ANY(s.issn)
			                       OR ji.eissn = ANY(s.issn)),
			              ARRAY[]::journal_kind[]
			          ) AS kinds
			   FROM sources s
			   WHERE s.id = $1"#,
		)
		.bind(id)
		.fetch_optional(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn save(&self, source: &Source) -> AppResult<SourceId> {
		let row = sqlx::query(
			"INSERT INTO sources (openalex_id, display_name, ty, issn_l, issn)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (openalex_id) DO UPDATE SET
            display_name = EXCLUDED.display_name,
            ty = EXCLUDED.ty,
            issn_l = EXCLUDED.issn_l,
            issn = EXCLUDED.issn RETURNING id",
		)
		.bind(&source.openalex_id)
		.bind(&source.display_name)
		.bind(&source.ty)
		.bind(&source.issn_l)
		.bind(&source.issn)
		.fetch_one(self.database.pool())
		.await?;

		Ok(SourceId::from_uuid(row.get("id")))
	}
}
