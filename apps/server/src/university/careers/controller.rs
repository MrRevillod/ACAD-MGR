use crate::university::*;

use std::sync::Arc;
use sword::prelude::*;
use sword::web::*;

#[controller(kind = ControllerKind::Web, path = "/careers")]
pub struct CareersController {
    careers: Arc<CareersService>,
}

impl CareersController {
    #[get("/")]
    pub async fn get_careers(&self, req: Request) -> WebResult<Vec<Career>> {
        let query = req.query_validator::<GetCareersQuery>()?;
        let careers = self.careers.find(query.unwrap_or_default()).await?;

        Ok(careers)
    }

    #[get("/{id}")]
    pub async fn get_career(&self, req: Request) -> WebResult<Career> {
        let career_id = req.param::<CareerId>("id")?;
        let career = self.careers.find_by_id(&career_id).await?;

        Ok(career)
    }

    #[post("/")]
    pub async fn create_career(&self, req: Request) -> WebResult<Career> {
        let input = req.body_validator::<CreateCareerDto>()?;
        let career = self.careers.create(input).await?;

        Ok(career)
    }
}
