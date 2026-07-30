use crate::research::stats::dtos::*;
use crate::research::stats::views::*;
use crate::shared::{AppResult, Database};

use sqlx::types::Uuid;
use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct StatsRepository {
	database: Arc<Database>,
}

impl StatsRepository {
	pub async fn stats_by_journal_kind(
		&self,
		query: &WorksStatsQuery,
	) -> AppResult<Vec<JournalKindRow>> {
		sqlx::query_as::<_, JournalKindRow>(
			r#"SELECT COALESCE((w.overrides).publication_year, w.publication_year) AS year,
					COUNT(DISTINCT w.id) FILTER (WHERE ji.kind = 'wos')::bigint AS wos,
					COUNT(DISTINCT w.id) FILTER (WHERE ji.kind = 'scopus')::bigint AS scopus
				FROM works w
				JOIN work_authorships wa ON w.id = wa.work_id AND wa.is_external = false
				JOIN academics a ON a.orcid = wa.orcid AND a.orcid != 'https://orcid.org/0000-0000-0000-0000'
				JOIN departments d ON a.department_id = d.id
				JOIN academic_category_options aco ON a.acad_category_options_id = aco.id
				LEFT JOIN sources src ON w.source_id = src.id
				LEFT JOIN LATERAL (
					SELECT kind FROM journal_issn WHERE issn = ANY(src.issn) LIMIT 1
				) ji ON TRUE
				WHERE COALESCE((w.overrides).publication_year, w.publication_year) >= $1
					AND ($2::smallint IS NULL OR COALESCE((w.overrides).publication_year, w.publication_year) <= $2)
					AND ($3::uuid IS NULL OR a.department_id = $3)
					AND ($4::academic_option IS NULL OR aco.option = $4)
					AND ($5::journal_kind IS NULL OR ji.kind = $5)
				GROUP BY year
				ORDER BY year"#,
		)
		.bind(query.year_from.unwrap_or(1900))
		.bind(query.year_to)
		.bind(query.department_id)
		.bind(query.option)
		.bind(query.journal_kind)
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn stats_by_option(&self, query: &WorksStatsQuery) -> AppResult<Vec<OptionRow>> {
		sqlx::query_as::<_, OptionRow>(
			r#"SELECT COALESCE((w.overrides).publication_year, w.publication_year) AS year,
					COUNT(DISTINCT w.id) FILTER (WHERE aco.option = 'teaching')::bigint AS teaching,
					COUNT(DISTINCT w.id) FILTER (WHERE aco.option = 'research')::bigint AS research
				FROM works w
				JOIN work_authorships wa ON w.id = wa.work_id AND wa.is_external = false
				JOIN academics a ON a.orcid = wa.orcid AND a.orcid != 'https://orcid.org/0000-0000-0000-0000'
				JOIN departments d ON a.department_id = d.id
				JOIN academic_category_options aco ON a.acad_category_options_id = aco.id
				LEFT JOIN sources src ON w.source_id = src.id
				LEFT JOIN LATERAL (
					SELECT kind FROM journal_issn WHERE issn = ANY(src.issn) LIMIT 1
				) ji ON TRUE
				WHERE COALESCE((w.overrides).publication_year, w.publication_year) >= $1
					AND ($2::smallint IS NULL OR COALESCE((w.overrides).publication_year, w.publication_year) <= $2)
					AND ($3::uuid IS NULL OR a.department_id = $3)
					AND ($4::academic_option IS NULL OR aco.option = $4)
					AND ($5::journal_kind IS NULL OR ji.kind = $5)
				GROUP BY year
				ORDER BY year"#,
		)
		.bind(query.year_from.unwrap_or(1900))
		.bind(query.year_to)
		.bind(query.department_id)
		.bind(query.option)
		.bind(query.journal_kind)
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn stats_by_department(
		&self,
		query: &WorksStatsQuery,
	) -> AppResult<Vec<DepartmentRow>> {
		sqlx::query_as::<_, DepartmentRow>(
			r#"SELECT COALESCE((w.overrides).publication_year, w.publication_year) AS year,
					d.id AS department_id,
					d.name AS department,
					COUNT(DISTINCT w.id)::bigint AS count
				FROM works w
				JOIN work_authorships wa ON w.id = wa.work_id AND wa.is_external = false
				JOIN academics a ON a.orcid = wa.orcid AND a.orcid != 'https://orcid.org/0000-0000-0000-0000'
				JOIN departments d ON a.department_id = d.id
				JOIN academic_category_options aco ON a.acad_category_options_id = aco.id
				LEFT JOIN sources src ON w.source_id = src.id
				LEFT JOIN LATERAL (
					SELECT kind FROM journal_issn WHERE issn = ANY(src.issn) LIMIT 1
				) ji ON TRUE
				WHERE COALESCE((w.overrides).publication_year, w.publication_year) >= $1
					AND ($2::smallint IS NULL OR COALESCE((w.overrides).publication_year, w.publication_year) <= $2)
					AND ($3::uuid IS NULL OR a.department_id = $3)
					AND ($4::academic_option IS NULL OR aco.option = $4)
					AND ($5::journal_kind IS NULL OR ji.kind = $5)
				GROUP BY year, d.id, d.name
				ORDER BY d.name, year"#,
		)
		.bind(query.year_from.unwrap_or(1900))
		.bind(query.year_to)
		.bind(query.department_id)
		.bind(query.option)
		.bind(query.journal_kind)
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn department_summary(
		&self,
		id: &Uuid,
		query: &DepartmentDetailQuery,
	) -> AppResult<DeptSummaryRow> {
		sqlx::query_as::<_, DeptSummaryRow>(
			r#"SELECT d.name AS department,
					COUNT(DISTINCT w.id)::bigint AS total,
					COUNT(DISTINCT w.id) FILTER (WHERE ji.kind = 'scopus')::bigint AS scopus,
					COUNT(DISTINCT w.id) FILTER (WHERE ji.kind = 'wos')::bigint AS wos,
					COUNT(DISTINCT w.id) FILTER (WHERE aco.option = 'teaching')::bigint AS teaching,
					COUNT(DISTINCT w.id) FILTER (WHERE aco.option = 'research')::bigint AS research
				FROM works w
				JOIN work_authorships wa ON w.id = wa.work_id AND wa.is_external = false
				JOIN academics a ON a.orcid = wa.orcid AND a.orcid != 'https://orcid.org/0000-0000-0000-0000'
				JOIN departments d ON a.department_id = d.id
				JOIN academic_category_options aco ON a.acad_category_options_id = aco.id
				LEFT JOIN sources src ON w.source_id = src.id
				LEFT JOIN LATERAL (
					SELECT kind FROM journal_issn WHERE issn = ANY(src.issn) LIMIT 1
				) ji ON TRUE
				WHERE d.id = $1
					AND COALESCE((w.overrides).publication_year, w.publication_year) >= $2
					AND ($3::smallint IS NULL OR COALESCE((w.overrides).publication_year, w.publication_year) <= $3)
					AND ($4::academic_option IS NULL OR aco.option = $4)
					AND ($5::journal_kind IS NULL OR ji.kind = $5)
				GROUP BY d.id, d.name"#,
		)
		.bind(id)
		.bind(query.year_from.unwrap_or(1900))
		.bind(query.year_to)
		.bind(query.option)
		.bind(query.journal_kind)
		.fetch_one(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn top_publishers(
		&self,
		id: &Uuid,
		query: &DepartmentDetailQuery,
	) -> AppResult<Vec<TopPublisherRow>> {
		sqlx::query_as::<_, TopPublisherRow>(
			r#"SELECT a.id AS academic_id,
					a.names || ' ' || a.paternal_surname || ' ' || a.maternal_surname AS name,
					COUNT(DISTINCT w.id)::bigint AS total,
					COUNT(DISTINCT w.id) FILTER (WHERE ji.kind = 'scopus')::bigint AS scopus,
					COUNT(DISTINCT w.id) FILTER (WHERE ji.kind = 'wos')::bigint AS wos,
					COUNT(DISTINCT w.id) FILTER (WHERE ji.kind IS NULL)::bigint AS unindexed,
					aco.option::text AS option
				FROM works w
				JOIN work_authorships wa ON w.id = wa.work_id AND wa.is_external = false
				JOIN academics a ON a.orcid = wa.orcid AND a.orcid != 'https://orcid.org/0000-0000-0000-0000'
				JOIN departments d ON a.department_id = d.id
				JOIN academic_category_options aco ON a.acad_category_options_id = aco.id
				LEFT JOIN sources src ON w.source_id = src.id
				LEFT JOIN LATERAL (
					SELECT kind FROM journal_issn WHERE issn = ANY(src.issn) LIMIT 1
				) ji ON TRUE
				WHERE d.id = $1
					AND COALESCE((w.overrides).publication_year, w.publication_year) >= $2
					AND ($3::smallint IS NULL OR COALESCE((w.overrides).publication_year, w.publication_year) <= $3)
					AND ($4::academic_option IS NULL OR aco.option = $4)
					AND ($5::journal_kind IS NULL OR ji.kind = $5)
				GROUP BY a.id, a.names, a.paternal_surname, a.maternal_surname, aco.option
				ORDER BY total DESC
				LIMIT 20"#,
		)
		.bind(id)
		.bind(query.year_from.unwrap_or(1900))
		.bind(query.year_to)
		.bind(query.option)
		.bind(query.journal_kind)
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}
}
