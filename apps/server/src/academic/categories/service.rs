use crate::{academic::*, shared::AppResult};
use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct AcademicCategoriesService {
    categories: Arc<AcademicCategoriesRepository>,
}

impl AcademicCategoriesService {
    pub async fn find(&self, query: GetCategoriesQuery) -> AppResult<Vec<AcademicCategory>> {
        let filter = AcademicCategoryFilter {
            name: query.name,
            planta: query.planta,
        };

        self.categories.list(filter).await
    }

    pub async fn find_by_id(&self, id: &AcademicCategoryId) -> AppResult<AcademicCategory> {
        let Some(category) = self.categories.find_by_id(id).await? else {
            return Err(AcademicError::CategoryNotFound)?;
        };

        Ok(category)
    }

    pub async fn create(&self, input: CreateAcademicCategoryDto) -> AppResult<AcademicCategory> {
        let category = AcademicCategory::builder()
            .name(input.name)
            .planta(input.planta)
            .build();

        self.categories.save(&category).await?;

        Ok(category)
    }
}
