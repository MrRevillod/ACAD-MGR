use crate::auth::SessionCheck;
use crate::university::*;

use std::sync::Arc;
use sword::prelude::*;
use sword::web::*;

#[controller(kind = ControllerKind::Web, path = "/departments")]
pub struct DepartmentsController {
    departments: Arc<DepartmentsService>,
}

impl DepartmentsController {
    #[get("/")]
    pub async fn get_departments(&self, req: Request) -> WebResult<Vec<Department>> {
        let query = req.query_validator::<GetDepartmentsQuery>()?;
        let departments = self.departments.find(query.unwrap_or_default()).await?;

        Ok(departments)
    }

    #[get("/{id}")]
    pub async fn get_department(&self, req: Request) -> WebResult<Department> {
        let department_id = req.param::<DepartmentId>("id")?;
        let department = self.departments.find_by_id(&department_id).await?;

        Ok(department)
    }

    #[post("/")]
    #[interceptor(SessionCheck)]
    pub async fn create_department(&self, req: Request) -> WebResult<Department> {
        let input = req.body_validator::<CreateDepartmentDto>()?;
        let department = self.departments.create(input).await?;

        Ok(department)
    }
}
