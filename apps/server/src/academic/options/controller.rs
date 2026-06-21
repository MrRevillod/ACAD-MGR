use crate::academic::*;
use crate::auth::SessionCheck;

use std::sync::Arc;
use sword::prelude::*;
use sword::web::*;

#[controller(kind = ControllerKind::Web, path = "/category-options")]
pub struct AcademicCategoryOptionsController {
    options: Arc<AcademicCategoryOptionsService>,
}

impl AcademicCategoryOptionsController {
    #[get("/")]
    pub async fn get_options(&self, req: Request) -> WebResult<Vec<AcademicCategoryOption>> {
        let query = req.query_validator::<GetCategoryOptionsQuery>()?;
        let options = self.options.find(query.unwrap_or_default()).await?;

        Ok(options)
    }

    #[get("/{id}")]
    pub async fn get_option(&self, req: Request) -> WebResult<AcademicCategoryOption> {
        let id = req.param::<AcademicCategoryOptionId>("id")?;
        let option = self.options.find_by_id(&id).await?;

        Ok(option)
    }

    #[post("/")]
    #[interceptor(SessionCheck)]
    pub async fn create_option(&self, req: Request) -> WebResult<AcademicCategoryOption> {
        let input = req.body_validator::<CreateCategoryOptionDto>()?;
        let option = self.options.create(input).await?;

        Ok(option)
    }
}
