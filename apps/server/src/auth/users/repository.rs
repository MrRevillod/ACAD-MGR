use crate::auth::*;
use crate::shared::{AppError, AppResult, Database};

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct UsersRepository {
	database: Arc<Database>,
}

impl UsersRepository {
	pub async fn list(&self, filter: UserFilter) -> AppResult<Vec<UserView>> {
		let mut query = User::all();

		if let Some(q) = filter.search {
			let pattern = format!("%{}%", q.trim());

			let name_pattern = User::fields().name().ilike(pattern.clone());
			let email_pattern = User::fields().email().ilike(pattern);

			query = query.filter(email_pattern.or(name_pattern));
		}

		if let Some(role) = filter.role {
			query = query.filter(User::fields().role().eq(role));
		}

		query = query.order_by(User::fields().name().asc()).limit(200);

		let users = query
			.exec(&mut self.database.pool())
			.await?
			.into_iter()
			.map(UserView::from)
			.collect();

		Ok(users)
	}

	pub async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
		User::filter_by_email(email)
			.first()
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)
	}

	pub async fn find_by_id(&self, id: &UserId) -> AppResult<Option<User>> {
		User::filter_by_id(id)
			.first()
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)
	}

	pub async fn delete(&self, id: &UserId) -> AppResult<()> {
		User::delete_by_id(&mut self.database.pool(), id)
			.await
			.map_err(AppError::from)
	}

	pub async fn create(&self, data: &CreateUserDto) -> AppResult<User> {
		User::create()
			.id(UserId::new())
			.name(&data.name)
			.email(&data.email)
			.password_hash(&data.password)
			.role(UserRole::Admin)
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)
	}

	pub async fn update(&self, id: &UserId, data: &UpdateUserDto) -> AppResult<()> {
		let mut builder = User::update_by_id(id);

		if let Some(name) = &data.name {
			builder = builder.name(name);
		}

		if let Some(email) = &data.email {
			builder = builder.email(email);
		}

		if let Some(password_hash) = &data.password {
			builder = builder.password_hash(password_hash);
		}

		if let Some(role) = &data.role {
			builder = builder.role(role);
		}

		builder
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)
	}
}
