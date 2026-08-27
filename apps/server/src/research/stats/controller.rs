use crate::academic::AcademicId;
use crate::research::*;
use crate::university::DepartmentId;

use std::sync::Arc;
use sword::prelude::*;
use sword::web::*;

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
		let id = req.param::<DepartmentId>("id")?;
		let query = req.query_validator::<DepartmentDetailQuery>()?;

		Ok(self
			.stats
			.get_department_detail(id, query.unwrap_or_default())
			.await?)
	}

	#[get("/academic/{id}")]
	pub async fn get_academic_stats(&self, req: Request) -> WebResult<AcademicStatsResponse> {
		let id = req.param::<AcademicId>("id")?;
		let query = req.query_validator::<AcademicStatsQuery>()?;

		Ok(self
			.stats
			.get_academic_stats(id, query.unwrap_or_default())
			.await?)
	}

	#[get("/research-line/{id}")]
	pub async fn get_research_line_stats(
		&self,
		req: Request,
	) -> WebResult<ResearchLineStatsResponse> {
		let id = req.param::<ResearchLineId>("id")?;
		let query = req.query_validator::<ResearchLineStatsQuery>()?;

		Ok(self
			.stats
			.get_research_line_stats(id, query.unwrap_or_default())
			.await?)
	}

	#[get("/productivity")]
	pub async fn get_productivity(&self, req: Request) -> WebResult<ProductivityResponse> {
		let query = req.query_validator::<ProductivityQuery>()?;

		Ok(self
			.stats
			.get_productivity(query.unwrap_or_default())
			.await?)
	}
}
