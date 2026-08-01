use crate::{
	academic::AcademicId,
	research::{CollaborationEdgeRow, CollaborationNodeRow, WorkId, WorkRef},
	shared::{AppResult, Database},
};
use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct CollaborationsRepository {
	database: Arc<Database>,
}

impl CollaborationsRepository {
	pub async fn find_nodes(
		&self,
		academic_id: &AcademicId,
	) -> AppResult<Vec<CollaborationNodeRow>> {
		sqlx::query_as::<_, CollaborationNodeRow>(
			"WITH focus AS (
				SELECT id, orcid FROM academics WHERE id = $1
			),
			coauthors AS (
				SELECT DISTINCT wa2.orcid
				FROM work_authorships wa1
				JOIN work_authorships wa2 ON wa2.work_id = wa1.work_id
				JOIN focus f ON f.orcid = wa1.orcid
				WHERE NOT wa1.is_external AND NOT wa2.is_external AND wa2.orcid <> f.orcid
			),
			ego AS (
				SELECT orcid FROM focus
				UNION
				SELECT orcid FROM coauthors
			)
			SELECT a.id, a.names, a.paternal_surname, a.maternal_surname,
				d.name AS department,
				COUNT(DISTINCT wa.work_id) AS total_works
			FROM academics a
			JOIN ego ON ego.orcid = a.orcid
			LEFT JOIN work_authorships wa ON wa.orcid = a.orcid AND NOT wa.is_external
			LEFT JOIN departments d ON d.id = a.department_id
			GROUP BY a.id, a.names, a.paternal_surname, a.maternal_surname, d.name",
		)
		.bind(academic_id)
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn find_edges(
		&self,
		academic_id: &AcademicId,
	) -> AppResult<Vec<CollaborationEdgeRow>> {
		sqlx::query_as::<_, CollaborationEdgeRow>(
			"WITH focus AS (
				SELECT id, orcid FROM academics WHERE id = $1
			),
			coauthors AS (
				SELECT DISTINCT wa2.orcid
				FROM work_authorships wa1
				JOIN work_authorships wa2 ON wa2.work_id = wa1.work_id
				JOIN focus f ON f.orcid = wa1.orcid
				WHERE NOT wa1.is_external AND NOT wa2.is_external AND wa2.orcid <> f.orcid
			),
			ego AS (
				SELECT orcid FROM focus
				UNION
				SELECT orcid FROM coauthors
			)
			SELECT a1.id AS source_id, a2.id AS target_id,
				COUNT(DISTINCT wa1.work_id) AS weight,
				array_agg(DISTINCT wa1.work_id) AS work_ids
			FROM work_authorships wa1
			JOIN work_authorships wa2 ON wa2.work_id = wa1.work_id AND wa2.orcid > wa1.orcid
			JOIN academics a1 ON a1.orcid = wa1.orcid
			JOIN academics a2 ON a2.orcid = wa2.orcid
			WHERE NOT wa1.is_external AND NOT wa2.is_external
				AND a1.orcid IN (SELECT orcid FROM ego)
				AND a2.orcid IN (SELECT orcid FROM ego)
			GROUP BY a1.id, a2.id",
		)
		.bind(academic_id)
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn find_works(&self, work_ids: &[WorkId]) -> AppResult<Vec<WorkRef>> {
		sqlx::query_as::<_, WorkRef>(
			"SELECT w.id, w.title,
				COALESCE((w.overrides).publication_year, w.publication_year) AS publication_year
			FROM works w
			WHERE w.id = ANY($1)
			ORDER BY COALESCE((w.overrides).publication_year, w.publication_year) DESC",
		)
		.bind(work_ids)
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}
}
