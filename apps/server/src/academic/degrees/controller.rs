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
    #[get("/")]
    pub async fn get_degrees(&self, req: Request) -> WebResult<Vec<Degree>> {
        let query = req.query_validator::<GetDegreesQuery>()?;
        let degrees = self.degrees.find(query.unwrap_or_default()).await?;

        Ok(degrees)
    }

    #[get("/{id}")]
    pub async fn get_degree(&self, req: Request) -> WebResult<Degree> {
        let id = req.param::<DegreeId>("id")?;
        let degree = self.degrees.find_by_id(&id).await?;

        Ok(degree)
    }

    #[post("/")]
    #[interceptor(SessionCheck)]
    pub async fn create_degree(&self, req: Request) -> WebResult<Degree> {
        let input = req.body_validator::<CreateDegreeDto>()?;
        let degree = self.degrees.create(input).await?;

        Ok(degree)
    }
}
