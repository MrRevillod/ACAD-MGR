use crate::auth::{User, UserFilter, UserId, UserView};
use crate::shared::{AppResult, Database};

use sqlx::QueryBuilder;
use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct UsersRepository {
    database: Arc<Database>,
}

impl UsersRepository {
    pub async fn list(&self, filter: UserFilter) -> AppResult<Vec<UserView>> {
        let mut query = QueryBuilder::new("SELECT id, name, email, role FROM users WHERE 1=1");

        if let Some(q) = filter.search {
            let pattern = format!("%{}%", q.trim());

            query
                .push(" AND (email ILIKE ")
                .push_bind(pattern.clone())
                .push(" OR name ILIKE ")
                .push_bind(pattern)
                .push(")");
        }

        if let Some(role) = filter.role {
            query.push(" AND role IN (");

            let mut separated = query.separated(", ");

            separated.push_bind(role);
            separated.push_unseparated(")");
        }

        query.push(" ORDER BY name ASC LIMIT 200");

        let users = query
            .build_query_as::<UserView>()
            .fetch_all(self.database.pool())
            .await?;

        Ok(users)
    }

    pub async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, name, email, role, password_hash FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(self.database.pool())
        .await?;

        Ok(user)
    }

    pub async fn find_by_id(&self, id: &UserId) -> AppResult<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, name, email, role, password_hash FROM users WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(self.database.pool())
        .await?;

        Ok(user)
    }

    pub async fn save(&self, user: &User) -> AppResult<User> {
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (id, name, email, role, password_hash) VALUES ($1, $2, $3, $4, $5)
             ON CONFLICT (id) DO UPDATE SET
                name = EXCLUDED.name,
                email = EXCLUDED.email,
                role = EXCLUDED.role,
                password_hash = EXCLUDED.password_hash
             RETURNING id, name, email, role, password_hash",
        )
        .bind(user.id)
        .bind(&user.name)
        .bind(&user.email)
        .bind(user.role)
        .bind(&user.password_hash)
        .fetch_one(self.database.pool())
        .await?;

        Ok(user)
    }
}
