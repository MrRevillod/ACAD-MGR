use crate::research::stats::*;
use crate::shared::AppResult;

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
			by_journal_kind: by_journal_kind?,
			by_option: by_option?,
			by_department: by_department?,
		})
	}

	pub async fn get_department_detail(
		&self,
		id: Uuid,
		query: DepartmentDetailQuery,
	) -> AppResult<DepartmentDetailResponse> {
		self.stats.department_detail(&id, &query).await
	}
}
