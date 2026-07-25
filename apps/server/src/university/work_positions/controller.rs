use crate::auth::SessionCheck;
use crate::university::*;

use std::sync::Arc;
use sword::prelude::*;
use sword::web::*;

#[controller(kind = ControllerKind::Web, path = "/work-positions")]
#[interceptor(SessionCheck)]
pub struct WorkPositionsController {
	positions: Arc<AcademicWorkPositionsService>,
}

impl WorkPositionsController {
	#[get("/")]
	pub async fn get_positions(&self, req: Request) -> WebResult<Vec<AcademicWorkPosition>> {
		Ok(self.positions.find().await?)
	}

	#[post("/")]
	pub async fn create_position(&self, req: Request) -> WebResult<AcademicWorkPosition> {
		let input = req.body_validator::<CreateAcademicWorkPositionDto>()?;
		let position = self.positions.create(input).await?;

		Ok(position)
	}
}
