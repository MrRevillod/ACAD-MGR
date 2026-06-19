use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Country {
    pub code: String,
    pub name: String,
}
