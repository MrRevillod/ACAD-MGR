use crate::auth::SessionCheck;
use crate::university::*;

use std::sync::Arc;
use sword::prelude::*;
use sword::web::*;

#[controller(kind = ControllerKind::Web, path = "/work-positions")]
pub struct WorkPositionsController {
    positions: Arc<AcademicWorkPositionsService>,
}

impl WorkPositionsController {
    #[get("/")]
    pub async fn get_positions(&self, req: Request) -> WebResult<Vec<AcademicWorkPosition>> {
        let query = req.query_validator::<GetWorkPositionsQuery>()?;
        let positions = self.positions.find(query.unwrap_or_default()).await?;

        Ok(positions)
    }

    #[get("/{code}")]
    pub async fn get_position(&self, req: Request) -> WebResult<AcademicWorkPosition> {
        let code = req.param::<String>("code")?;
        let position = self.positions.find_by_code(&code).await?;

        Ok(position)
    }

    #[post("/")]
    #[interceptor(SessionCheck)]
    pub async fn create_position(&self, req: Request) -> WebResult<AcademicWorkPosition> {
        let input = req.body_validator::<CreateAcademicWorkPositionDto>()?;
        let position = self.positions.create(input).await?;

        Ok(position)
    }
}
