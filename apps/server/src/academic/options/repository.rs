use crate::academic::options::AcademicCategoryOption;
use crate::shared::{AppResult, Tx};
use sword::prelude::*;

#[injectable]
pub struct AcademicCategoryOptionsRepository;

impl AcademicCategoryOptionsRepository {
    pub async fn save(
        &self,
        tx: &mut Tx<'_>,
        option: &AcademicCategoryOption,
    ) -> AppResult<()> {
        sqlx::query(
            "INSERT INTO academic_category_options (id, category_code, option) VALUES ($1, $2, $3)",
        )
        .bind(&option.id)
        .bind(&option.category_code)
        .bind(&option.option_code)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}
