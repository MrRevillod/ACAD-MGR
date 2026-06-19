use crate::shared::{AppResult, Tx};
use crate::university::work_positions::AcademicWorkPosition;
use sword::prelude::*;

#[injectable]
pub struct AcademicWorkPositionsRepository;

impl AcademicWorkPositionsRepository {
    pub async fn save(&self, tx: &mut Tx<'_>, position: &AcademicWorkPosition) -> AppResult<()> {
        sqlx::query("INSERT INTO academic_work_positions (code, name) VALUES ($1, $2)")
            .bind(&position.code)
            .bind(&position.name)
            .execute(&mut **tx)
            .await?;

        Ok(())
    }
}
