use crate::auth::SessionCheck;
use crate::university::*;

use std::sync::Arc;
use sword::prelude::*;
use sword::web::*;

#[controller(kind = ControllerKind::Web, path = "/faculties")]
pub struct FacultiesController {
    faculties: Arc<FacultiesService>,
}

impl FacultiesController {
    #[get("/")]
    pub async fn get_faculties(&self, req: Request) -> WebResult<Vec<Faculty>> {
        let query = req.query_validator::<GetFacultiesQuery>()?;
        let faculties = self.faculties.find(query.unwrap_or_default()).await?;

        Ok(faculties)
    }

    #[get("/{id}")]
    pub async fn get_faculty(&self, req: Request) -> WebResult<Faculty> {
        let faculty_id = req.param::<FacultyId>("id")?;
        let faculty = self.faculties.find_by_id(&faculty_id).await?;

        Ok(faculty)
    }

    #[post("/")]
    #[interceptor(SessionCheck)]
    pub async fn create_faculty(&self, req: Request) -> WebResult<Faculty> {
        let input = req.body_validator::<CreateFacultyDto>()?;
        let faculty = self.faculties.create(input).await?;

        Ok(faculty)
    }
}
