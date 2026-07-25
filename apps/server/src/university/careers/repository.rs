use crate::shared::{AppError, AppResult, Database};
use crate::university::{Career, CareerFilter, CareerId};

use std::sync::Arc;
use sword::prelude::*;
use toasty::schema::Model;

#[injectable]
pub struct CareersRepository {
	database: Arc<Database>,
}

impl CareersRepository {
	pub async fn list(&self, filter: CareerFilter) -> AppResult<Vec<Career>> {
		let mut query = Career::all();

		if let Some(n) = filter.name {
			query = query.filter(Career::fields().name().ilike(format!("%{}%", n.trim())));
		}

		if let Some(dept_id) = filter.department_id {
			query = query.filter(Career::fields().department_id().eq(dept_id));
		}

		query.exec(&mut self.database.pool()).await.into()
	}

	pub async fn find_by_id(&self, id: &CareerId) -> AppResult<Option<Career>> {
		Career::get_by_id(&mut self.database.pool(), id)
			.await?
			.map_err(AppError::from)
	}

	pub async fn create(&self, career: &Career) -> AppResult<()> {
		Career::create()
			.id(career.id.clone())
			.name(career.name.clone())
			.department_id(career.department_id.clone())
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)?;

		Ok(())
	}
}
