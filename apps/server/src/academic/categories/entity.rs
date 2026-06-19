use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
#[sqlx(type_name = "academic_option", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum AcademicOption {
    Teaching,
    Research,
}

#[derive(Debug, Clone, Type, Serialize, Deserialize)]
#[sqlx(type_name = "academic_planta", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum AcademicPlanta {
    Adjunta,
    Permanente,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AcademicCategory {
    pub code: String,
    pub name: String,
    pub planta: AcademicPlanta,
}
