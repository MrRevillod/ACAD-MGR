use crate::academic::categories::AcademicOption;
use crate::shared::{Entity, Id};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub type AcademicCategoryOptionId = Id<AcademicCategoryOption>;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AcademicCategoryOption {
    pub id: AcademicCategoryOptionId,
    pub category_code: String,
    pub option_code: AcademicOption,
}

impl Entity for AcademicCategoryOption {
    fn key_name() -> &'static str {
        "academic_category_option"
    }
}
