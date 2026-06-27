use crate::shared::{AppResult, Database};
use crate::university::{AcademicWorkPosition, AcademicWorkPositionId, WorkPositionFilter};

use sqlx::{Postgres, QueryBuilder};
use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct AcademicWorkPositionsRepository {
    database: Arc<Database>,
}

impl AcademicWorkPositionsRepository {
    pub async fn list(&self, filter: WorkPositionFilter) -> AppResult<Vec<AcademicWorkPosition>> {
        let mut query =
            QueryBuilder::<Postgres>::new("SELECT id, name FROM academic_work_positions WHERE 1=1");

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

    pub async fn find_by_id(
        &self,
        id: &AcademicWorkPositionId,
    ) -> AppResult<Option<AcademicWorkPosition>> {
        let item = sqlx::query_as::<_, AcademicWorkPosition>(
            "SELECT id, name FROM academic_work_positions WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(self.database.pool())
        .await?;

        Ok(item)
    }

    pub async fn find_by_name(&self, name: &str) -> AppResult<Option<AcademicWorkPosition>> {
        let item = sqlx::query_as::<_, AcademicWorkPosition>(
            "SELECT id, name FROM academic_work_positions WHERE name = $1",
        )
        .bind(name)
        .fetch_optional(self.database.pool())
        .await?;

        Ok(item)
    }

    pub async fn save(&self, position: &AcademicWorkPosition) -> AppResult<()> {
        sqlx::query("INSERT INTO academic_work_positions (id, name) VALUES ($1, $2)")
            .bind(position.id)
            .bind(&position.name)
            .execute(self.database.pool())
            .await?;

        Ok(())
    }
}
