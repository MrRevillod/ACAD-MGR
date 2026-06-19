use crate::academic::degrees::Degree;
use crate::shared::{AppResult, Tx};
use sword::prelude::*;

#[injectable]
pub struct DegreesRepository;

impl DegreesRepository {
    pub async fn save(&self, tx: &mut Tx<'_>, degree: &Degree) -> AppResult<()> {
        sqlx::query(
            "INSERT INTO degrees (id, academic_id, name, university, obtained_at, kind, country_code) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(&degree.id)
        .bind(&degree.academic_id)
        .bind(&degree.name)
        .bind(&degree.university)
        .bind(&degree.obtained_at)
        .bind(&degree.kind)
        .bind(&degree.country_code)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}
