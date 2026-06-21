use crate::academic::*;
use crate::auth::SessionCheck;

use std::sync::Arc;
use sword::prelude::*;
use sword::web::*;

#[controller(kind = ControllerKind::Web, path = "/academic-categories")]
pub struct AcademicCategoriesController {
    categories: Arc<AcademicCategoriesService>,
}

impl AcademicCategoriesController {
    #[get("/")]
    pub async fn get_categories(&self, req: Request) -> WebResult<Vec<AcademicCategory>> {
        let query = req.query_validator::<GetCategoriesQuery>()?;
        let categories = self.categories.find(query.unwrap_or_default()).await?;

        Ok(categories)
    }

    #[get("/{id}")]
    pub async fn get_category(&self, req: Request) -> WebResult<AcademicCategory> {
        let id = req.param::<AcademicCategoryId>("id")?;
        let category = self.categories.find_by_id(&id).await?;

        Ok(category)
    }

    #[post("/")]
    #[interceptor(SessionCheck)]
    pub async fn create_category(&self, req: Request) -> WebResult<AcademicCategory> {
        let input = req.body_validator::<CreateAcademicCategoryDto>()?;
        let category = self.categories.create(input).await?;

        Ok(category)
    }
}
