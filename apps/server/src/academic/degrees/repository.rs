use crate::academic::degrees::Degree;
use crate::shared::{AppResult, Database, Tx};

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct DegreesRepository {
    database: Arc<Database>,
}

impl DegreesRepository {
    pub async fn save(&self, degree: &Degree) -> AppResult<()> {
        sqlx::query(
            "INSERT INTO degrees (id, academic_id, name, university, obtained_at, kind, country_code) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(degree.id)
        .bind(degree.academic_id)
        .bind(&degree.name)
        .bind(&degree.university)
        .bind(degree.obtained_at)
        .bind(&degree.kind)
        .bind(&degree.country_code)
        .execute(self.database.get_pool())
        .await?;

        Ok(())
    }

    pub async fn save_tx(&self, tx: &mut Tx<'_>, degree: &Degree) -> AppResult<()> {
        sqlx::query(
			"INSERT INTO degrees (id, academic_id, name, university, obtained_at, kind, country_code) VALUES ($1, $2, $3, $4, $5, $6, $7)",
		)
		.bind(degree.id)
		.bind(degree.academic_id)
		.bind(&degree.name)
		.bind(&degree.university)
		.bind(degree.obtained_at)
		.bind(&degree.kind)
		.bind(&degree.country_code)
		.execute(&mut **tx)
		.await?;

        Ok(())
    }
}
