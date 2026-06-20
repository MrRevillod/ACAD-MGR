use crate::university::FacultyId;

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateDepartmentDto {
    #[validate(length(
        min = 1,
        max = 255,
        message = "El nombre del departamento debe tener entre 1 y 255 caracteres"
    ))]
    pub name: String,

    pub faculty_id: FacultyId,
}

#[derive(Debug, Serialize, Deserialize, Validate, Default)]
pub struct GetDepartmentsQuery {
    #[validate(length(
        min = 1,
        max = 255,
        message = "El nombre del departamento debe tener entre 1 y 255 caracteres"
    ))]
    pub name: Option<String>,

    pub faculty_id: Option<FacultyId>,
}
