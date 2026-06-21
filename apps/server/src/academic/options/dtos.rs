use crate::academic::{AcademicCategoryId, AcademicOption};

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateCategoryOptionDto {
    pub category_id: AcademicCategoryId,
    pub option: AcademicOption,
}

#[derive(Debug, Serialize, Deserialize, Validate, Default)]
pub struct GetCategoryOptionsQuery {
    pub category_id: Option<AcademicCategoryId>,
}
