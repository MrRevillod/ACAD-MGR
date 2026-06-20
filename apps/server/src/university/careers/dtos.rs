use crate::university::DepartmentId;

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateCareerDto {
    #[validate(length(
        min = 1,
        max = 255,
        message = "El nombre de la carrera debe tener entre 1 y 255 caracteres"
    ))]
    pub name: String,

    pub department_id: DepartmentId,
}

#[derive(Debug, Serialize, Deserialize, Validate, Default)]
pub struct GetCareersQuery {
    #[validate(length(
        min = 1,
        max = 255,
        message = "El nombre de la carrera debe tener entre 1 y 255 caracteres"
    ))]
    pub name: Option<String>,
    pub department_id: Option<DepartmentId>,
}
