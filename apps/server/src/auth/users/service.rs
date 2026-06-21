use crate::auth::{
    AuthError, CreateUserDto, GetUsersQuery, Hasher, User, UserFilter, UserId, UserView,
    UsersRepository,
};
use crate::shared::AppResult;

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct UsersService {
    users: Arc<UsersRepository>,
    hasher: Arc<Hasher>,
}

impl UsersService {
    pub async fn find(&self, query: GetUsersQuery) -> AppResult<Vec<UserView>> {
        let filter = UserFilter {
            role: query.role,
            search: query.search,
        };

        self.users.list(filter).await
    }

    pub async fn create(&self, dto: CreateUserDto) -> AppResult<UserView> {
        if self.users.find_by_email(&dto.email).await?.is_some() {
            return Err(AuthError::EmailAlreadyExists)?;
        }

        let user = User {
            id: UserId::new(),
            name: dto.name,
            email: dto.email,
            role: dto.role,
            password_hash: self.hasher.hash(&dto.password)?,
        };

        let user = self.users.save(&user).await?;

        Ok(UserView {
            id: user.id,
            name: user.name,
            email: user.email,
            role: user.role,
        })
    }
}
