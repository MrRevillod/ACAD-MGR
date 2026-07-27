use crate::academic::{AcademicCategory, AcademicCategoryFilter, AcademicCategoryId};
use crate::shared::{AppError, AppResult, Database};

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct AcademicCategoriesRepository {
	database: Arc<Database>,
}

impl AcademicCategoriesRepository {
	pub async fn list(&self, filter: AcademicCategoryFilter) -> AppResult<Vec<AcademicCategory>> {
		let mut categories = AcademicCategory::all();

		if let Some(n) = filter.name {
			let pattern = format!("%{}%", n.trim());
			categories = categories.filter(AcademicCategory::fields().name().ilike(pattern));
		}

		if let Some(planta) = filter.planta {
			categories = categories.filter(AcademicCategory::fields().planta().eq(planta));
		}

		categories
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)
	}

	pub async fn find_by_id(&self, id: &AcademicCategoryId) -> AppResult<Option<AcademicCategory>> {
		AcademicCategory::filter_by_id(id)
			.first()
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)
	}

	pub async fn save(&self, category: &AcademicCategory) -> AppResult<()> {
		AcademicCategory::create()
			.id(&category.id)
			.name(&category.name)
			.planta(category.planta)
			.exec(&mut self.database.pool())
			.await?;

		Ok(())
	}
}
