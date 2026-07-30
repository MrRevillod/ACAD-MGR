use crate::research::*;
use crate::shared::AppResult;

use std::collections::BTreeMap;
use std::sync::Arc;
use sword::prelude::*;
use uuid::Uuid;

#[injectable]
pub struct StatsService {
	stats: Arc<StatsRepository>,
}

impl StatsService {
	pub async fn get_works_stats(&self, query: WorksStatsQuery) -> AppResult<WorksStatsResponse> {
		let (by_journal_kind, by_option, by_department) = tokio::join!(
			self.stats.stats_by_journal_kind(&query),
			self.stats.stats_by_option(&query),
			self.stats.stats_by_department(&query),
		);

		Ok(WorksStatsResponse {
			by_journal_kind: Self::build_journal_kind_series(by_journal_kind?),
			by_option: Self::build_option_series(by_option?),
			by_department: Self::build_department_series(by_department?),
		})
	}

	pub async fn get_department_detail(
		&self,
		id: Uuid,
		query: DepartmentDetailQuery,
	) -> AppResult<DepartmentDetailResponse> {
		let (summary, publishers) = tokio::join!(
			self.stats.department_summary(&id, &query),
			self.stats.top_publishers(&id, &query),
		);

		let summary = summary.map_err(|_| StatsError::DepartmentNotFound(id))?;
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

	fn build_journal_kind_series(rows: Vec<JournalKindRow>) -> Vec<TimeSeriesStat> {
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

		vec![
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
		]
	}

	fn build_option_series(rows: Vec<OptionRow>) -> Vec<TimeSeriesStat> {
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

		vec![
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
		]
	}

	fn build_department_series(rows: Vec<DepartmentRow>) -> Vec<TimeSeriesStat> {
		let mut map: BTreeMap<String, (Option<String>, Vec<YearValue>)> = BTreeMap::new();

		for r in &rows {
			let entry = map.entry(r.department.clone()).or_default();
			entry.0 = Some(r.department_id.to_string());
			entry.1.push(YearValue {
				year: r.year,
				value: r.count.unwrap_or(0),
			});
		}

		map.into_iter()
			.map(|(key, (id, values))| TimeSeriesStat { id, key, values })
			.collect()
	}
}
