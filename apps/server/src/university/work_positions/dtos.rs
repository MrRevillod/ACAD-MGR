use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateAcademicWorkPositionDto {
    #[validate(length(
        min = 1,
        max = 50,
        message = "El código debe tener entre 1 y 50 caracteres"
    ))]
    pub code: String,

    #[validate(length(
        min = 1,
        max = 255,
        message = "El nombre debe tener entre 1 y 255 caracteres"
    ))]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, Default)]
pub struct GetWorkPositionsQuery {
    #[validate(length(
        min = 1,
        max = 50,
        message = "El código debe tener entre 1 y 50 caracteres"
    ))]
    pub code: Option<String>,

    #[validate(length(
        min = 1,
        max = 255,
        message = "El nombre debe tener entre 1 y 255 caracteres"
    ))]
    pub name: Option<String>,
}
