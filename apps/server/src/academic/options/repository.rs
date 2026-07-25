use crate::academic::*;
use crate::shared::{AppError, AppResult, Database};

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct AcademicCategoryOptionsRepository {
	database: Arc<Database>,
}

impl AcademicCategoryOptionsRepository {
	pub async fn list(
		&self,
		filter: AcademicCategoryOptionFilter,
	) -> AppResult<Vec<AcademicCategoryOption>> {
		let mut options = AcademicCategoryOption::all();

		if let Some(cid) = filter.category_id {
			options = options.filter(AcademicCategoryOption::fields().category_id.eq(cid));
		}

		options
			.exec(&mut self.database.pool())
			.await?
			.map_err(AppError::from)
	}

	pub async fn find_one(
		&self,
		filter: AcademicCategoryOptionFilter,
	) -> AppResult<Option<AcademicCategoryOption>> {
		let mut option = AcademicCategoryOption::all();

		if let Some(cid) = filter.category_id {
			option = option.filter(AcademicCategoryOption::fields().category_id.eq(cid));
		}

		if let Some(option) = filter.option {
			option = option.filter(AcademicCategoryOption::fields().option.eq(option));
		}

		option
			.first()
			.exec(&mut self.database.pool())
			.await?
			.map_err(AppError::from)
	}

	pub async fn find_by_id(
		&self,
		id: &AcademicCategoryOptionId,
	) -> AppResult<Option<AcademicCategoryOption>> {
		AcademicCategoryOption::get_by_id(&mut self.database.pool(), id)
			.await?
			.map_err(AppError::from)
	}

	pub async fn save(&self, option: &AcademicCategoryOption) -> AppResult<()> {
		AcademicCategoryOption::create()
			.id(&option.id)
			.category_id(&option.category_id)
			.option(option.option)
			.hours(option.hours)
			.execute(&mut self.database.pool())
			.await?
			.map_err(AppError::from)
	}
}
