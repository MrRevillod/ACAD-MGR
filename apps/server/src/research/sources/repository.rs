use crate::research::sources::views::SourceView;
use crate::research::*;
use crate::shared::{AppResult, Database};

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct SourcesRepository {
	database: Arc<Database>,
}

impl SourcesRepository {
	pub async fn find_source_view_by_id(&self, id: &SourceId) -> AppResult<Option<SourceView>> {
		sqlx::query_as::<_, SourceView>(
			r#"SELECT s.id, s.openalex_id, s.name, s.ty, s.issn,
			          (SELECT ji.kind FROM journal_issn ji
			           WHERE ji.issn = ANY(s.issn)
			           LIMIT 1
			          ) AS kind
			   FROM sources s
			   WHERE s.id = $1"#,
		)
		.bind(id)
		.fetch_optional(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn find_by_openalex_id(&self, openalex_id: &str) -> AppResult<Option<Source>> {
		sqlx::query_as::<_, Source>(
			"SELECT id, openalex_id, name, ty, issn, journal_issn_id FROM sources WHERE openalex_id = $1",
		)
		.bind(openalex_id)
		.fetch_optional(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn save(&self, source: &Source) -> AppResult<()> {
		sqlx::query(
			"INSERT INTO sources (id, openalex_id, name, ty, issn, journal_issn_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (id) DO UPDATE SET
            name = EXCLUDED.name,
            ty = EXCLUDED.ty,
            issn = EXCLUDED.issn,
            journal_issn_id = EXCLUDED.journal_issn_id",
		)
		.bind(source.id)
		.bind(&source.openalex_id)
		.bind(&source.name)
		.bind(&source.ty)
		.bind(&source.issn)
		.bind(source.journal_issn_id)
		.execute(self.database.pool())
		.await?;

		Ok(())
	}

	pub async fn find_kinds_by_source_ids(
		&self,
		ids: &[SourceId],
	) -> AppResult<std::collections::HashMap<SourceId, Option<JournalKind>>> {
		if ids.is_empty() {
			return Ok(std::collections::HashMap::new());
		}

		let rows = sqlx::query_as::<_, (SourceId, Option<JournalKind>)>(
			r#"SELECT s.id,
			          (SELECT ji.kind FROM journal_issn ji
			           WHERE ji.issn = ANY(s.issn)
			           LIMIT 1
			          ) AS kind
			   FROM sources s
			   WHERE s.id = ANY($1)"#,
		)
		.bind(ids)
		.fetch_all(self.database.pool())
		.await?;

		Ok(rows.into_iter().collect())
	}
}
