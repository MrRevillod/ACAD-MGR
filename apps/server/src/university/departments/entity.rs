use crate::shared::{Entity, Id};
use crate::university::FacultyId;
use sqlx::FromRow;

pub type DepartmentId = Id<Department>;

#[derive(Debug, Clone, FromRow)]
pub struct Department {
    pub id: DepartmentId,
    pub name: String,
    pub faculty_id: FacultyId,
}

impl Entity for Department {
    fn key_name() -> &'static str {
        "department"
    }
}
