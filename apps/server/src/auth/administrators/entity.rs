use crate::shared::{Entity, Id};
use sqlx::FromRow;

pub type AdministratorId = Id<Administrator>;

#[derive(Debug, Clone, FromRow)]
pub struct Administrator {
    pub id: AdministratorId,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

impl Entity for Administrator {
    fn key_name() -> &'static str {
        "administrator"
    }
}
