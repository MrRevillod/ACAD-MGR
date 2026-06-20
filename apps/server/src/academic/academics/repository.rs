use std::sync::Arc;

use crate::academic::academics::Academic;
use crate::shared::{AppResult, Database, Tx};
use sword::prelude::*;

#[injectable]
pub struct AcademicsRepository {
    database: Arc<Database>,
}

impl AcademicsRepository {
    pub async fn save(&self, tx: &mut Tx<'_>, academic: &Academic) -> AppResult<()> {
        let query = r#"
        INSERT INTO academics (
			id, rut, names, paternal_surname, maternal_surname, email, orcid, sex,
			birth_date, joined_at, work_position_code, work_position_details,
			department_id, career_id, uct_working_hours, acad_category_options_id,
			acad_category_hours, annual_discount_hours, nationality_code, city
		) VALUES (
			$1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
			$11, $12, $13, $14, $15, $16, $17, $18, $19, $20
		)"#;

        sqlx::query(query)
            .bind(academic.id)
            .bind(&academic.rut)
            .bind(&academic.names)
            .bind(&academic.paternal_surname)
            .bind(&academic.maternal_surname)
            .bind(&academic.email)
            .bind(&academic.orcid)
            .bind(academic.sex)
            .bind(academic.birth_date)
            .bind(academic.joined_at)
            .bind(&academic.work_position_code)
            .bind(&academic.work_position_details)
            .bind(academic.department_id)
            .bind(academic.career_id)
            .bind(academic.uct_working_hours)
            .bind(academic.acad_category_options_id)
            .bind(academic.acad_category_hours)
            .bind(academic.annual_discount_hours)
            .bind(&academic.nationality_code)
            .bind(&academic.city)
            .execute(&mut **tx)
            .await?;

        Ok(())
    }

    pub async fn find_by_rut(&self, rut: &str) -> AppResult<Option<Academic>> {
        let item = sqlx::query_as::<_, Academic>("SELECT * FROM academics WHERE rut = $1")
            .bind(rut)
            .fetch_optional(self.database.get_pool())
            .await?;

        Ok(item)
    }
}
