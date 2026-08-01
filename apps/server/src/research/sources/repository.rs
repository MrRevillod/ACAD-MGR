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
			"SELECT s.id, s.openalex_id, s.name, s.ty, s.issn, ji.kind AS kind
			FROM sources s
			LEFT JOIN journal_issn ji ON ji.issn = s.issn
			WHERE s.id = $1",
		)
		.bind(id)
		.fetch_optional(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn resolve_issn(&self, issns: &[String]) -> AppResult<Option<String>> {
		sqlx::query_scalar::<_, String>(
			"SELECT issn FROM journal_issn
			WHERE issn = ANY($1)
			ORDER BY (kind = 'wos') DESC
			LIMIT 1",
		)
		.bind(issns)
		.fetch_optional(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn find_by_openalex_id(&self, openalex_id: &str) -> AppResult<Option<Source>> {
		sqlx::query_as::<_, Source>(
			"SELECT id, openalex_id, name, ty, issn FROM sources
			WHERE openalex_id = $1",
		)
		.bind(openalex_id)
		.fetch_optional(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn save(&self, source: &Source) -> AppResult<()> {
		sqlx::query(
			"INSERT INTO sources (id, openalex_id, name, ty, issn)
			VALUES ($1, $2, $3, $4, $5)
		    ON CONFLICT (id) DO UPDATE SET
		        name = EXCLUDED.name,
		        ty   = EXCLUDED.ty,
		        issn = EXCLUDED.issn",
		)
		.bind(source.id)
		.bind(&source.openalex_id)
		.bind(&source.name)
		.bind(&source.ty)
		.bind(&source.issn)
		.execute(self.database.pool())
		.await?;

		Ok(())
	}
}
