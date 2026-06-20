use std::sync::Arc;

use crate::academic::options::AcademicCategoryOption;
use crate::shared::{AppResult, Database, Tx};
use sword::prelude::*;

#[injectable]
pub struct AcademicCategoryOptionsRepository {
    database: Arc<Database>,
}

impl AcademicCategoryOptionsRepository {
    pub async fn save(&self, tx: &mut Tx<'_>, option: &AcademicCategoryOption) -> AppResult<()> {
        sqlx::query(
            "INSERT INTO academic_category_options (id, category_code, option) VALUES ($1, $2, $3)",
        )
        .bind(option.id)
        .bind(&option.category_code)
        .bind(option.option_code)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn list(&self) -> AppResult<Vec<AcademicCategoryOption>> {
        let items = sqlx::query_as::<_, AcademicCategoryOption>(
            "SELECT id, category_code, option FROM academic_category_options",
        )
        .fetch_all(self.database.get_pool())
        .await?;

        Ok(items)
    }
}
