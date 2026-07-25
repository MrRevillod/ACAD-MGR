use crate::auth::*;
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

	pub async fn find_by_id(&self, id: &UserId) -> AppResult<UserView> {
		let Some(user) = self.users.find_by_id(id).await? else {
			return Err(AuthError::UserNotFound)?;
		};

		Ok(UserView::from(user))
	}

	pub async fn create(&self, mut dto: CreateUserDto) -> AppResult<UserView> {
		if self.users.find_by_email(&dto.email).await?.is_some() {
			Err(AuthError::EmailAlreadyExists)?;
		}

		dto.password = self.hasher.hash(&dto.password)?;

		let user = self.users.create(&dto).await?;

		Ok(UserView::from(user))
	}

	pub async fn update(&self, id: &UserId, mut dto: UpdateUserDto) -> AppResult<UserView> {
		let Some(user) = self.users.find_by_id(id).await? else {
			return Err(AuthError::UserNotFound)?;
		};

		if let Some(ref email) = dto.email
			&& email != &user.email
			&& self.users.find_by_email(email).await?.is_some()
		{
			Err(AuthError::EmailAlreadyExists)?;
		}

		if let Some(ref password) = dto.password {
			dto.password = Some(self.hasher.hash(password)?);
		}

		let user = self.users.update(&user.id, &dto).await?;

		Ok(UserView::from(user))
	}

	pub async fn delete(&self, id: &UserId) -> AppResult<()> {
		if self.users.find_by_id(id).await?.is_none() {
			Err(AuthError::UserNotFound)?;
		}

		self.users.delete(id).await
	}
}
