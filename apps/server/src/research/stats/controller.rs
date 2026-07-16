use crate::research::*;

use std::sync::Arc;
use sword::prelude::*;
use sword::web::*;

use uuid::Uuid;

#[controller(kind = ControllerKind::Web, path = "/stats")]
pub struct StatsController {
	stats: Arc<StatsService>,
}

impl StatsController {
	#[get("/works")]
	pub async fn get_works_stats(&self, req: Request) -> WebResult<WorksStatsResponse> {
		let query = req.query_validator::<WorksStatsQuery>()?;

		Ok(self
			.stats
			.get_works_stats(query.unwrap_or_default())
			.await?)
	}

	#[get("/department/{id}")]
	pub async fn get_department_detail(&self, req: Request) -> WebResult<DepartmentDetailResponse> {
		let id = req.param::<Uuid>("id")?;
		let query = req.query_validator::<DepartmentDetailQuery>()?;

		Ok(self
			.stats
			.get_department_detail(id, query.unwrap_or_default())
			.await?)
	}
}
