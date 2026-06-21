use crate::academic::degrees::Degree;
use crate::academic::{AcademicId, DegreeId};
use crate::shared::{AppResult, Database, Tx};

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct DegreesRepository {
    database: Arc<Database>,
}

impl DegreesRepository {
    pub async fn list(&self, academic_id: Option<AcademicId>) -> AppResult<Vec<Degree>> {
        match academic_id {
            Some(aid) => {
                let items = sqlx::query_as::<_, Degree>(
                    "SELECT id, academic_id, name, university, obtained_at, kind, country_code
                     FROM degrees WHERE academic_id = $1 ORDER BY obtained_at DESC",
                )
                .bind(aid)
                .fetch_all(self.database.pool())
                .await?;
                Ok(items)
            }
            None => {
                let items = sqlx::query_as::<_, Degree>(
                    "SELECT id, academic_id, name, university, obtained_at, kind, country_code
                     FROM degrees ORDER BY obtained_at DESC",
                )
                .fetch_all(self.database.pool())
                .await?;
                Ok(items)
            }
        }
    }

    pub async fn find_by_id(&self, id: &DegreeId) -> AppResult<Option<Degree>> {
        let item = sqlx::query_as::<_, Degree>(
            "SELECT id, academic_id, name, university, obtained_at, kind, country_code
             FROM degrees WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(self.database.pool())
        .await?;

        Ok(item)
    }

    pub async fn save(&self, degree: &Degree) -> AppResult<()> {
        sqlx::query(
            "INSERT INTO degrees (id, academic_id, name, university, obtained_at, kind, country_code)
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(degree.id)
        .bind(degree.academic_id)
        .bind(&degree.name)
        .bind(&degree.university)
        .bind(degree.obtained_at)
        .bind(&degree.kind)
        .bind(&degree.country_code)
        .execute(self.database.pool())
        .await?;

        Ok(())
    }

    pub async fn save_tx(&self, tx: &mut Tx<'_>, degree: &Degree) -> AppResult<()> {
        sqlx::query(
            "INSERT INTO degrees (id, academic_id, name, university, obtained_at, kind, country_code)
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
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
