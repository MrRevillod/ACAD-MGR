use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct AcademicWorkPosition {
    pub code: String,
    pub name: String,
}
