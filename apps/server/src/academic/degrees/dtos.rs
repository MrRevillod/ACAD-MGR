use crate::academic::{AcademicId, DegreeKind};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateDegreeDto {
    pub academic_id: AcademicId,

    #[validate(length(min = 1, max = 255, message = "El nombre debe tener entre 1 y 255 caracteres"))]
    pub name: String,

    #[validate(length(min = 1, max = 255, message = "La universidad debe tener entre 1 y 255 caracteres"))]
    pub university: String,

    pub obtained_at: NaiveDate,
    pub kind: DegreeKind,

    #[validate(length(min = 2, max = 2, message = "El código de país debe tener 2 caracteres"))]
    pub country_code: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, Default)]
pub struct GetDegreesQuery {
    pub academic_id: Option<AcademicId>,
}
