use crate::academic::*;
use crate::auth::SessionCheck;

use std::sync::Arc;
use sword::prelude::*;
use sword::web::*;

#[controller(kind = ControllerKind::Web, path = "/academics")]
pub struct AcademicsController {
    academics: Arc<AcademicsService>,
}

impl AcademicsController {
    #[get("/")]
    pub async fn get_academics(&self, req: Request) -> WebResult<Vec<AcademicView>> {
        let query = req.query_validator::<GetAcademicsQuery>()?;
        let academics = self.academics.find(query.unwrap_or_default()).await?;

        Ok(academics)
    }

    #[get("/{id}")]
    pub async fn get_academic(&self, req: Request) -> WebResult<AcademicView> {
        let id = req.param::<AcademicId>("id")?;
        let academic = self.academics.find_view_by_id(&id).await?;

        Ok(academic)
    }

    #[post("/")]
    #[interceptor(SessionCheck)]
    pub async fn create_academic(&self, req: Request) -> WebResult<AcademicView> {
        let input = req.body_validator::<CreateAcademicDto>()?;
        let academic = self.academics.create(input).await?;

        Ok(academic)
    }
}
