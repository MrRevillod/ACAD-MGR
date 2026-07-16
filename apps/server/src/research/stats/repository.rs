use crate::research::stats::*;
use crate::shared::{AppResult, Database};

use sqlx::FromRow;
use sqlx::types::Uuid;
use std::sync::Arc;
use sword::prelude::*;

#[derive(Debug, FromRow)]
struct JournalKindRow {
	year: i16,
	wos: Option<i64>,
	scopus: Option<i64>,
}

#[derive(Debug, FromRow)]
struct OptionRow {
	year: i16,
	teaching: Option<i64>,
	research: Option<i64>,
}

#[derive(Debug, FromRow)]
struct DepartmentRow {
	year: i16,
	department_id: Uuid,
	department: String,
	count: Option<i64>,
}

#[derive(Debug, FromRow)]
struct TopPublisherRow {
	academic_id: Uuid,
	name: String,
	total: Option<i64>,
	scopus: Option<i64>,
	wos: Option<i64>,
	unindexed: Option<i64>,
	option: String,
}

#[derive(Debug, FromRow)]
struct DeptSummaryRow {
	department: String,
	total: Option<i64>,
	scopus: Option<i64>,
	wos: Option<i64>,
	teaching: Option<i64>,
	research: Option<i64>,
}

fn base_from() -> &'static str {
	r#"
	FROM works w
	JOIN work_authorships wa ON w.id = wa.work_id
		AND wa.is_external = false
	JOIN academics a ON a.orcid = wa.orcid
		AND a.orcid != 'https://orcid.org/0000-0000-0000-0000'
	JOIN departments d ON a.department_id = d.id
	JOIN academic_category_options aco ON a.acad_category_options_id = aco.id
	LEFT JOIN sources src ON w.primary_source_id = src.id
	LEFT JOIN LATERAL (
		SELECT kind FROM journal_issn
		WHERE issn = src.issn_l OR eissn = src.issn_l
			OR issn = ANY(src.issn) OR eissn = ANY(src.issn)
		LIMIT 1
	) ji ON TRUE
	WHERE w.publication_year >= $1
		AND ($2::smallint IS NULL OR w.publication_year <= $2)
		AND ($3::uuid IS NULL OR a.department_id = $3)
		AND ($4::academic_option IS NULL OR aco.option = $4)
		AND ($5::journal_kind IS NULL OR ji.kind = $5)
	"#
}

#[injectable]
pub struct StatsRepository {
	database: Arc<Database>,
}

impl StatsRepository {
	pub async fn stats_by_journal_kind(
		&self,
		query: &WorksStatsQuery,
	) -> AppResult<Vec<TimeSeriesStat>> {
		let rows = sqlx::query_as::<_, JournalKindRow>(&format!(
			r#"SELECT w.publication_year AS year,
					COUNT(DISTINCT w.id) FILTER (WHERE ji.kind = 'wos')::bigint AS wos,
					COUNT(DISTINCT w.id) FILTER (WHERE ji.kind = 'scopus')::bigint AS scopus
				{}
				GROUP BY w.publication_year
				ORDER BY w.publication_year"#,
			base_from()
		))
		.bind(query.year_from.unwrap_or(1900))
		.bind(query.year_to)
		.bind(query.department_id)
		.bind(query.option)
		.bind(query.journal_kind)
		.fetch_all(self.database.pool())
		.await?;

		let mut scopus_vals = Vec::new();
		let mut wos_vals = Vec::new();

		for r in &rows {
			scopus_vals.push(YearValue {
				year: r.year,
				value: r.scopus.unwrap_or(0),
			});
			wos_vals.push(YearValue {
				year: r.year,
				value: r.wos.unwrap_or(0),
			});
		}

		Ok(vec![
			TimeSeriesStat {
				id: None,
				key: "scopus".into(),
				values: scopus_vals,
			},
			TimeSeriesStat {
				id: None,
				key: "wos".into(),
				values: wos_vals,
			},
		])
	}

	pub async fn stats_by_option(&self, query: &WorksStatsQuery) -> AppResult<Vec<TimeSeriesStat>> {
		let rows = sqlx::query_as::<_, OptionRow>(&format!(
			r#"SELECT w.publication_year AS year,
					COUNT(DISTINCT w.id) FILTER (WHERE aco.option = 'teaching')::bigint AS teaching,
					COUNT(DISTINCT w.id) FILTER (WHERE aco.option = 'research')::bigint AS research
				{}
				GROUP BY w.publication_year
				ORDER BY w.publication_year"#,
			base_from()
		))
		.bind(query.year_from.unwrap_or(1900))
		.bind(query.year_to)
		.bind(query.department_id)
		.bind(query.option)
		.bind(query.journal_kind)
		.fetch_all(self.database.pool())
		.await?;

		let mut teaching_vals = Vec::new();
		let mut research_vals = Vec::new();

		for r in &rows {
			teaching_vals.push(YearValue {
				year: r.year,
				value: r.teaching.unwrap_or(0),
			});
			research_vals.push(YearValue {
				year: r.year,
				value: r.research.unwrap_or(0),
			});
		}

		Ok(vec![
			TimeSeriesStat {
				id: None,
				key: "teaching".into(),
				values: teaching_vals,
			},
			TimeSeriesStat {
				id: None,
				key: "research".into(),
				values: research_vals,
			},
		])
	}

	pub async fn stats_by_department(
		&self,
		query: &WorksStatsQuery,
	) -> AppResult<Vec<TimeSeriesStat>> {
		let rows = sqlx::query_as::<_, DepartmentRow>(&format!(
			r#"SELECT w.publication_year AS year,
					d.id AS department_id,
					d.name AS department,
					COUNT(DISTINCT w.id)::bigint AS count
				{}
				GROUP BY w.publication_year, d.id, d.name
				ORDER BY d.name, w.publication_year"#,
			base_from()
		))
		.bind(query.year_from.unwrap_or(1900))
		.bind(query.year_to)
		.bind(query.department_id)
		.bind(query.option)
		.bind(query.journal_kind)
		.fetch_all(self.database.pool())
		.await?;

		let mut map: std::collections::BTreeMap<String, (Option<String>, Vec<YearValue>)> =
			std::collections::BTreeMap::new();

		for r in &rows {
			let entry = map.entry(r.department.clone()).or_default();
			entry.0 = Some(r.department_id.to_string());
			entry.1.push(YearValue {
				year: r.year,
				value: r.count.unwrap_or(0),
			});
		}

		let stats = map
			.into_iter()
			.map(|(key, (id, values))| TimeSeriesStat { id, key, values })
			.collect();

		Ok(stats)
	}

	pub async fn department_detail(
		&self,
		id: &Uuid,
		query: &DepartmentDetailQuery,
	) -> AppResult<DepartmentDetailResponse> {
		let pool = self.database.pool();

		let summary_query = sqlx::query_as::<_, DeptSummaryRow>(
			r#"
			SELECT d.name AS department,
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
			LEFT JOIN sources src ON w.primary_source_id = src.id
			LEFT JOIN LATERAL (
				SELECT kind FROM journal_issn WHERE issn = src.issn_l OR eissn = src.issn_l
					OR issn = ANY(src.issn) OR eissn = ANY(src.issn) LIMIT 1
			) ji ON TRUE
			WHERE d.id = $1 AND w.publication_year >= $2
				AND ($3::smallint IS NULL OR w.publication_year <= $3)
				AND ($4::academic_option IS NULL OR aco.option = $4)
				AND ($5::journal_kind IS NULL OR ji.kind = $5)
			GROUP BY d.id, d.name
			"#,
		)
		.bind(id)
		.bind(query.year_from.unwrap_or(1900))
		.bind(query.year_to)
		.bind(query.option)
		.bind(query.journal_kind);

		let publishers_query = sqlx::query_as::<_, TopPublisherRow>(
			r#"
			SELECT
				a.id AS academic_id,
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
			LEFT JOIN sources src ON w.primary_source_id = src.id
			LEFT JOIN LATERAL (
				SELECT kind FROM journal_issn WHERE issn = src.issn_l OR eissn = src.issn_l
					OR issn = ANY(src.issn) OR eissn = ANY(src.issn) LIMIT 1
			) ji ON TRUE
			WHERE d.id = $1 AND w.publication_year >= $2
				AND ($3::smallint IS NULL OR w.publication_year <= $3)
				AND ($4::academic_option IS NULL OR aco.option = $4)
				AND ($5::journal_kind IS NULL OR ji.kind = $5)
			GROUP BY a.id, a.names, a.paternal_surname, a.maternal_surname, aco.option
			ORDER BY total DESC
			LIMIT 20
			"#,
		)
		.bind(id)
		.bind(query.year_from.unwrap_or(1900))
		.bind(query.year_to)
		.bind(query.option)
		.bind(query.journal_kind);

		let (summary, publishers) = tokio::join!(
			summary_query.fetch_one(pool),
			publishers_query.fetch_all(pool),
		);
		let summary = summary?;
		let publishers = publishers?;

		let top_publishers = publishers
			.into_iter()
			.map(|r| TopPublisher {
				academic_id: r.academic_id.to_string(),
				name: r.name,
				total: r.total.unwrap_or(0),
				scopus: r.scopus.unwrap_or(0),
				wos: r.wos.unwrap_or(0),
				unindexed: r.unindexed.unwrap_or(0),
				option: r.option,
			})
			.collect();

		Ok(DepartmentDetailResponse {
			department: summary.department,
			total_works: summary.total.unwrap_or(0),
			scopus_count: summary.scopus.unwrap_or(0),
			wos_count: summary.wos.unwrap_or(0),
			teaching_count: summary.teaching.unwrap_or(0),
			research_count: summary.research.unwrap_or(0),
			top_publishers,
		})
	}
}
