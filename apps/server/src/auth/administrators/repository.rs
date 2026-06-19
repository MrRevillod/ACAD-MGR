use crate::auth::administrators::Administrator;
use crate::shared::{AppResult, Tx};
use sword::prelude::*;

#[injectable]
pub struct AdministratorsRepository;

impl AdministratorsRepository {
    pub async fn save(&self, tx: &mut Tx<'_>, user: &Administrator) -> AppResult<()> {
        sqlx::query(
            "INSERT INTO administrators (id, name, email, password_hash) VALUES ($1, $2, $3, $4)",
        )
        .bind(&user.id)
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.password_hash)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}
