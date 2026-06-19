use crate::shared::{Entity, Id};
use sqlx::FromRow;

pub type FacultyId = Id<Faculty>;

#[derive(Debug, Clone, FromRow)]
pub struct Faculty {
    pub id: FacultyId,
    pub name: String,
}

impl Entity for Faculty {
    fn key_name() -> &'static str {
        "faculty"
    }
}
