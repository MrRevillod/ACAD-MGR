use crate::shared::{AppError, AppResult, Database};
use crate::university::{Department, DepartmentFilter, DepartmentId};

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct DepartmentsRepository {
	database: Arc<Database>,
}

impl DepartmentsRepository {
	pub async fn list(&self, filter: DepartmentFilter) -> AppResult<Vec<Department>> {
		let mut query = Department::all();

		if let Some(n) = filter.name {
			query = query.filter(Department::fields().name().ilike(format!("%{}%", n.trim())));
		}

		if let Some(faculty_id) = filter.faculty_id {
			query = query.filter(Department::fields().faculty_id().eq(faculty_id));
		}

		query
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)
	}

	pub async fn find_by_id(&self, id: &DepartmentId) -> AppResult<Option<Department>> {
		Department::filter_by_id(id)
			.first()
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)
	}

	pub async fn find_by_name(&self, name: &str) -> AppResult<Option<Department>> {
		Department::all()
			.filter(Department::fields().name().eq(name))
			.first()
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)
	}

	pub async fn save(&self, department: &Department) -> AppResult<()> {
		Department::create()
			.id(department.id)
			.name(&department.name)
			.faculty_id(department.faculty_id)
			.exec(&mut self.database.pool())
			.await?;

		Ok(())
	}
}
