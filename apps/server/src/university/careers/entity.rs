use crate::shared::{Entity, Id};
use crate::university::DepartmentId;
use sqlx::FromRow;

pub type CareerId = Id<Career>;

#[derive(Debug, Clone, FromRow)]
pub struct Career {
    pub id: CareerId,
    pub name: String,
    pub department_id: DepartmentId,
}

impl Entity for Career {
    fn key_name() -> &'static str {
        "career"
    }
}
