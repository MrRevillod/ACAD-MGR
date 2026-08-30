use crate::academic::AcademicId;
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
			"SELECT wa.*, a.id AS academic_id FROM work_authorships wa
				LEFT JOIN academics a ON a.orcid = wa.orcid
			WHERE wa.work_id = $1
			ORDER BY CASE wa.position WHEN 'first' THEN 0 WHEN 'middle' THEN 1 WHEN 'last' THEN 2
			END",
		)
		.bind(work_id)
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn save(&self, authorship: &Authorship) -> AppResult<()> {
		sqlx::query(
			"INSERT INTO work_authorships (
				work_id, orcid, name, is_external, is_corresponding, affiliations, position
			) VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT (work_id, orcid) DO NOTHING",
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

	pub async fn delete_for_orcid_not_in(&self, orcid: &str, keep: &[WorkId]) -> AppResult<usize> {
		let result = sqlx::query(
			"DELETE FROM work_authorships
			WHERE orcid = $1 AND NOT (work_id = ANY($2))",
		)
		.bind(orcid)
		.bind(keep)
		.execute(self.database.pool())
		.await?;

		Ok(result.rows_affected() as usize)
	}

	pub async fn delete_for_orcid(&self, orcid: &str) -> AppResult<usize> {
		let result = sqlx::query("DELETE FROM work_authorships WHERE orcid = $1")
			.bind(orcid)
			.execute(self.database.pool())
			.await?;

		Ok(result.rows_affected() as usize)
	}

	pub async fn exists_local_author(
		&self,
		work_id: &WorkId,
		academic_id: &AcademicId,
	) -> AppResult<bool> {
		let exists = sqlx::query_scalar::<_, bool>(
			"SELECT EXISTS(
				SELECT 1 FROM work_authorships wa
				JOIN academics a ON a.orcid = wa.orcid
				WHERE wa.work_id = $1 AND a.id = $2
			)",
		)
		.bind(work_id)
		.bind(academic_id)
		.fetch_one(self.database.pool())
		.await?;

		Ok(exists)
	}

	pub async fn update_affiliations(
		&self,
		work_id: &WorkId,
		orcid: &str,
		affiliations: &[String],
	) -> AppResult<bool> {
		let result = sqlx::query(
			"UPDATE work_authorships SET affiliations = $3
			WHERE work_id = $1 AND orcid = $2",
		)
		.bind(work_id)
		.bind(orcid)
		.bind(affiliations)
		.execute(self.database.pool())
		.await?;

		Ok(result.rows_affected() > 0)
	}
}
