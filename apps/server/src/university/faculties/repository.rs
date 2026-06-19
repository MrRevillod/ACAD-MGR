use crate::shared::{AppResult, Tx};
use crate::university::faculties::Faculty;
use sword::prelude::*;

#[injectable]
pub struct FacultiesRepository;

impl FacultiesRepository {
    pub async fn save(&self, tx: &mut Tx<'_>, faculty: &Faculty) -> AppResult<()> {
        sqlx::query("INSERT INTO faculties (id, name) VALUES ($1, $2)")
            .bind(&faculty.id)
            .bind(&faculty.name)
            .execute(&mut **tx)
            .await?;

        Ok(())
    }
}
