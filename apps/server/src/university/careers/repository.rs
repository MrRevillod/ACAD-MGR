use crate::shared::{AppResult, Tx};
use crate::university::careers::Career;
use sword::prelude::*;

#[injectable]
pub struct CareersRepository;

impl CareersRepository {
    pub async fn save(&self, tx: &mut Tx<'_>, career: &Career) -> AppResult<()> {
        sqlx::query("INSERT INTO careers (id, name, department_id) VALUES ($1, $2, $3)")
            .bind(&career.id)
            .bind(&career.name)
            .bind(&career.department_id)
            .execute(&mut **tx)
            .await?;

        Ok(())
    }
}
