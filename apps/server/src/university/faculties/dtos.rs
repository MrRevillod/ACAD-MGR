use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateFacultyDto {
    #[validate(length(
        min = 1,
        max = 255,
        message = "El nombre de la facultad debe tener entre 1 y 255 caracteres"
    ))]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, Default)]
pub struct GetFacultiesQuery {
    #[validate(length(
        min = 1,
        max = 255,
        message = "El nombre de la facultad debe tener entre 1 y 255 caracteres"
    ))]
    pub name: Option<String>,
}
