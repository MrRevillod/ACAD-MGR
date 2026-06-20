use std::sync::Arc;

use crate::academic::categories::AcademicCategory;
use crate::shared::{AppResult, Database, Tx};
use sword::prelude::*;

#[injectable]
pub struct AcademicCategoriesRepository {
    database: Arc<Database>,
}

impl AcademicCategoriesRepository {
    pub async fn save(&self, tx: &mut Tx<'_>, category: &AcademicCategory) -> AppResult<()> {
        sqlx::query("INSERT INTO academic_categories (code, name, planta) VALUES ($1, $2, $3)")
            .bind(&category.code)
            .bind(&category.name)
            .bind(&category.planta)
            .execute(&mut **tx)
            .await?;

        Ok(())
    }

    pub async fn list(&self) -> AppResult<Vec<AcademicCategory>> {
        let items = sqlx::query_as::<_, AcademicCategory>(
            "SELECT code, name, planta FROM academic_categories",
        )
        .fetch_all(self.database.get_pool())
        .await?;

        Ok(items)
    }
}
