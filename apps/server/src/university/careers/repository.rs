use std::sync::Arc;

use crate::shared::{AppResult, Database, Tx};
use crate::university::careers::Career;
use crate::university::{CareerFilter, CareerId};
use sqlx::{Postgres, QueryBuilder};
use sword::prelude::*;

#[injectable]
pub struct CareersRepository {
    database: Arc<Database>,
}

impl CareersRepository {
    pub async fn list(&self, filter: CareerFilter) -> AppResult<Vec<Career>> {
        let mut query =
            QueryBuilder::<Postgres>::new("SELECT id, name, department_id FROM careers WHERE 1=1");

        if let Some(n) = filter.name {
            let pattern = format!("%{}%", n.trim());

            query
                .push(" AND (name ILIKE ")
                .push_bind(pattern.clone())
                .push(")");
        }

        if let Some(dept_id) = filter.department_id {
            query.push(" AND department_id = ").push_bind(dept_id);
        }

        let careers = query
            .build_query_as::<Career>()
            .fetch_all(self.database.get_pool())
            .await?;

        Ok(careers)
    }

    pub async fn find_by_id(&self, id: &CareerId) -> AppResult<Option<Career>> {
        let item = sqlx::query_as::<_, Career>(
            "SELECT id, name, department_id FROM careers WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(self.database.get_pool())
        .await?;

        Ok(item)
    }

    pub async fn save(&self, career: &Career) -> AppResult<()> {
        sqlx::query("INSERT INTO careers (id, name, department_id) VALUES ($1, $2, $3)")
            .bind(career.id)
            .bind(&career.name)
            .bind(career.department_id)
            .execute(self.database.get_pool())
            .await?;

        Ok(())
    }

    pub async fn _save_with_tx(&self, tx: &mut Tx<'_>, career: &Career) -> AppResult<()> {
        sqlx::query("INSERT INTO careers (id, name, department_id) VALUES ($1, $2, $3)")
            .bind(career.id)
            .bind(&career.name)
            .bind(career.department_id)
            .execute(&mut **tx)
            .await?;

        Ok(())
    }
}
