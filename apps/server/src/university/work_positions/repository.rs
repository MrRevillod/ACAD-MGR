use std::sync::Arc;

use crate::shared::{AppResult, Database};
use crate::university::WorkPositionFilter;
use crate::university::work_positions::AcademicWorkPosition;
use sqlx::{Postgres, QueryBuilder};
use sword::prelude::*;

#[injectable]
pub struct AcademicWorkPositionsRepository {
    database: Arc<Database>,
}

impl AcademicWorkPositionsRepository {
    pub async fn list(&self, filter: WorkPositionFilter) -> AppResult<Vec<AcademicWorkPosition>> {
        let mut query = QueryBuilder::<Postgres>::new(
            "SELECT code, name FROM academic_work_positions WHERE 1=1",
        );

        if let Some(c) = filter.code {
            let pattern = format!("%{}%", c.trim());

            query
                .push(" AND (code ILIKE ")
                .push_bind(pattern.clone())
                .push(")");
        }

        if let Some(n) = filter.name {
            let pattern = format!("%{}%", n.trim());

            query
                .push(" AND (name ILIKE ")
                .push_bind(pattern.clone())
                .push(")");
        }

        let positions = query
            .build_query_as::<AcademicWorkPosition>()
            .fetch_all(self.database.pool())
            .await?;

        Ok(positions)
    }

    pub async fn find_by_code(&self, code: &str) -> AppResult<Option<AcademicWorkPosition>> {
        let item = sqlx::query_as::<_, AcademicWorkPosition>(
            "SELECT code, name FROM academic_work_positions WHERE code = $1",
        )
        .bind(code)
        .fetch_optional(self.database.pool())
        .await?;

        Ok(item)
    }

    pub async fn save(&self, position: &AcademicWorkPosition) -> AppResult<()> {
        sqlx::query("INSERT INTO academic_work_positions (code, name) VALUES ($1, $2)")
            .bind(&position.code)
            .bind(&position.name)
            .execute(self.database.pool())
            .await?;

        Ok(())
    }
}
