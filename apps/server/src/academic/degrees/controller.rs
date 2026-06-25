use crate::academic::*;
use crate::auth::SessionCheck;

use std::sync::Arc;
use sword::prelude::*;
use sword::web::*;

#[controller(kind = ControllerKind::Web, path = "/degrees")]
pub struct DegreesController {
    degrees: Arc<DegreesService>,
}

impl DegreesController {
    #[get("/{academic_id}")]
    pub async fn get_degrees(&self, req: Request) -> WebResult<Vec<Degree>> {
        let academic_id = req.param::<AcademicId>("academic_id")?;
        let degrees = self.degrees.find(&academic_id).await?;

        Ok(degrees)
    }

    #[post("/")]
    #[interceptor(SessionCheck)]
    pub async fn create_degree(&self, req: Request) -> WebResult<Degree> {
        let input = req.body_validator::<CreateDegreeDto>()?;
        let degree = self.degrees.create(input).await?;

        Ok(degree)
    }
}
