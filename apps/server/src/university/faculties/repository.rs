use crate::shared::{AppError, AppResult, Database};
use crate::university::{Faculty, FacultyFilter, FacultyId};

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct FacultiesRepository {
	database: Arc<Database>,
}

impl FacultiesRepository {
	pub async fn list(&self, filter: FacultyFilter) -> AppResult<Vec<Faculty>> {
		let query = Faculty::all();

		if let Some(n) = filter.name {
			query = query.filter(Faculty::fields().name().ilike(format!("%{}%", n.trim())))
		}

		query
			.exec(&mut self.database.pool())
			.await?
			.map_err(AppError::from)
	}

	pub async fn find_by_id(&self, id: &FacultyId) -> AppResult<Option<Faculty>> {
		Faculty::get_by_id(&mut self.database.pool(), id)
			.await?
			.map_err(AppError::from)
	}

	pub async fn save(&self, faculty: &Faculty) -> AppResult<()> {
		Faculty::create()
			.id(faculty.id)
			.name(faculty.name.clone())
			.exec(&mut self.database.pool())
			.await?
			.map_err(AppError::from)
	}
}
