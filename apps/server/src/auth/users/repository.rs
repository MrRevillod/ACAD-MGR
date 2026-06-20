use crate::auth::{User, UserId};
use crate::shared::{AppResult, Database};

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct UsersRepository {
    database: Arc<Database>,
}

impl UsersRepository {
    pub async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let admin = sqlx::query_as::<_, User>(
            "SELECT id, name, email, role, password_hash FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(self.database.get_pool())
        .await?;

        Ok(admin)
    }

    pub async fn find_by_id(&self, id: &UserId) -> AppResult<Option<User>> {
        let admin = sqlx::query_as::<_, User>(
            "SELECT id, name, email, password_hash FROM users WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(self.database.get_pool())
        .await?;

        Ok(admin)
    }

    pub async fn save(&self, user: &User) -> AppResult<()> {
        sqlx::query(
            "INSERT INTO users (id, name, email, role, password_hash) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(user.id)
        .bind(&user.name)
        .bind(&user.email)
        .bind(user.role)
        .bind(&user.password_hash)
        .execute(self.database.get_pool())
        .await?;

        Ok(())
    }
}
