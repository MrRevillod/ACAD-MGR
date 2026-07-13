use crate::research::*;
use crate::shared::{AppResult, Database};

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct AuthorshipsRepository {
	database: Arc<Database>,
}

impl AuthorshipsRepository {
	pub async fn list(&self, work_id: &WorkId) -> AppResult<Vec<Authorship>> {
		sqlx::query_as::<_, Authorship>(
			"SELECT * FROM work_authorships WHERE work_id = $1
            ORDER BY CASE position WHEN 'first' THEN 0 WHEN 'middle' THEN 1 WHEN 'last' THEN 2 END",
		)
		.bind(work_id)
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn insert(&self, authorship: &NewAuthorship) -> AppResult<()> {
		sqlx::query(
			"INSERT INTO work_authorships (
            	work_id, orcid, name, is_external,
               	is_corresponding, affiliations, position
            ) VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT (work_id, orcid) DO NOTHING
        ",
		)
		.bind(authorship.work_id)
		.bind(&authorship.orcid)
		.bind(&authorship.name)
		.bind(authorship.is_external)
		.bind(authorship.is_corresponding)
		.bind(&authorship.affiliations)
		.bind(authorship.position)
		.execute(self.database.pool())
		.await?;

		Ok(())
	}
}
