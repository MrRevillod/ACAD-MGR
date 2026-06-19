use crate::academic::categories::AcademicCategory;
use crate::shared::{AppResult, Tx};
use sword::prelude::*;

#[injectable]
pub struct AcademicCategoriesRepository;

impl AcademicCategoriesRepository {
    pub async fn save_category(
        &self,
        tx: &mut Tx<'_>,
        category: &AcademicCategory,
    ) -> AppResult<()> {
        sqlx::query("INSERT INTO academic_categories (code, name, planta) VALUES ($1, $2, $3)")
            .bind(&category.code)
            .bind(&category.name)
            .bind(&category.planta)
            .execute(&mut **tx)
            .await?;

        Ok(())
    }
}
