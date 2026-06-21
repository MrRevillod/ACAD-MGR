use crate::academic::{Academic, AcademicId, AcademicSortField, AcademicView};
use crate::shared::{AppResult, Database};

use sqlx::QueryBuilder;
use std::sync::Arc;
use sword::prelude::*;

#[derive(Debug)]
pub struct AcademicListFilter {
    pub search: Option<String>,
    pub sort: Option<AcademicSortField>,
}

#[injectable]
pub struct AcademicsRepository {
    database: Arc<Database>,
}

impl AcademicsRepository {
    pub async fn list(&self, filter: AcademicListFilter) -> AppResult<Vec<AcademicView>> {
        let mut query = QueryBuilder::new(
            r#"
            SELECT
                a.id, a.rut, a.names, a.paternal_surname, a.maternal_surname,
                a.email, a.orcid, a.sex, a.birth_date, a.joined_at,
                wp.name AS work_position, a.work_position_details,
                d.name AS department,
                c.name AS career,
                a.uct_working_hours,
                ac.name AS category,
                ac.planta,
                aco.option,
                a.acad_category_hours, a.annual_discount_hours,
                co.name AS nationality,
                a.city
            FROM academics a
            LEFT JOIN academic_work_positions wp ON a.work_position_code = wp.code
            JOIN departments d ON a.department_id = d.id
            LEFT JOIN careers c ON a.career_id = c.id
            JOIN academic_category_options aco ON a.acad_category_options_id = aco.id
            JOIN academic_categories ac ON aco.category_id = ac.id
            JOIN countries co ON a.nationality_code = co.code
            WHERE 1=1
            "#,
        );

        if let Some(q) = filter.search {
            let pattern = format!("%{}%", q.trim());

            query
                .push(" AND (a.names ILIKE ")
                .push_bind(pattern.clone())
                .push(" OR a.paternal_surname ILIKE ")
                .push_bind(pattern.clone())
                .push(" OR a.maternal_surname ILIKE ")
                .push_bind(pattern.clone())
                .push(" OR a.rut ILIKE ")
                .push_bind(pattern.clone())
                .push(" OR a.email ILIKE ")
                .push_bind(pattern)
                .push(")");
        }

        match filter.sort {
            Some(AcademicSortField::Names) => {
                query.push(" ORDER BY a.names ASC");
            }
            Some(AcademicSortField::PaternalSurname) => {
                query.push(" ORDER BY a.paternal_surname, a.maternal_surname, a.names ASC");
            }
            Some(AcademicSortField::MaternalSurname) => {
                query.push(" ORDER BY a.maternal_surname, a.paternal_surname, a.names ASC");
            }
            Some(AcademicSortField::JoinedAt) => {
                query.push(" ORDER BY a.joined_at ASC");
            }
            Some(AcademicSortField::BirthDate) => {
                query.push(" ORDER BY a.birth_date ASC");
            }
            None => {
                query.push(" ORDER BY a.paternal_surname, a.maternal_surname, a.names ASC");
            }
        }

        let items = query
            .build_query_as::<AcademicView>()
            .fetch_all(self.database.pool())
            .await?;

        Ok(items)
    }

    pub async fn find_view_by_id(&self, id: &AcademicId) -> AppResult<Option<AcademicView>> {
        let item = sqlx::query_as::<_, AcademicView>(
            r#"
            SELECT
                a.id, a.rut, a.names, a.paternal_surname, a.maternal_surname,
                a.email, a.orcid, a.sex, a.birth_date, a.joined_at,
                wp.name AS work_position, a.work_position_details,
                d.name AS department,
                c.name AS career,
                a.uct_working_hours,
                ac.name AS category,
                ac.planta,
                aco.option,
                a.acad_category_hours, a.annual_discount_hours,
                co.name AS nationality,
                a.city
            FROM academics a
            LEFT JOIN academic_work_positions wp ON a.work_position_code = wp.code
            JOIN departments d ON a.department_id = d.id
            LEFT JOIN careers c ON a.career_id = c.id
            JOIN academic_category_options aco ON a.acad_category_options_id = aco.id
            JOIN academic_categories ac ON aco.category_id = ac.id
            JOIN countries co ON a.nationality_code = co.code
            WHERE a.id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(self.database.pool())
        .await?;

        Ok(item)
    }

    pub async fn find_by_id(&self, id: &AcademicId) -> AppResult<Option<Academic>> {
        let item = sqlx::query_as::<_, Academic>("SELECT * FROM academics WHERE id = $1")
            .bind(id)
            .fetch_optional(self.database.pool())
            .await?;

        Ok(item)
    }

    pub async fn find_by_rut(&self, rut: &str) -> AppResult<Option<Academic>> {
        let item = sqlx::query_as::<_, Academic>("SELECT * FROM academics WHERE rut = $1")
            .bind(rut)
            .fetch_optional(self.database.pool())
            .await?;

        Ok(item)
    }

    pub async fn save(&self, academic: &Academic) -> AppResult<()> {
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
            .execute(self.database.pool())
            .await?;

        Ok(())
    }
}
