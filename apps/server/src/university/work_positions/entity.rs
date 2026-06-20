use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct AcademicWorkPosition {
    pub code: String,
    pub name: String,
}

pub struct WorkPositionFilter {
    pub name: Option<String>,
    pub code: Option<String>,
}
