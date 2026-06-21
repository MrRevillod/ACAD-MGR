use std::sync::Arc;

use crate::shared::{AppResult, Database};
use crate::university::departments::Department;
use crate::university::{DepartmentFilter, DepartmentId};
use sqlx::{Postgres, QueryBuilder};
use sword::prelude::*;

#[injectable]
pub struct DepartmentsRepository {
    database: Arc<Database>,
}

impl DepartmentsRepository {
    pub async fn list(&self, filter: DepartmentFilter) -> AppResult<Vec<Department>> {
        let mut query =
            QueryBuilder::<Postgres>::new("SELECT id, name, faculty_id FROM departments WHERE 1=1");

        tracing::info!("Building query with filter: {:?}", filter);

        if let Some(n) = filter.name {
            let pattern = format!("%{}%", n.trim());

            query
                .push(" AND (name ILIKE ")
                .push_bind(pattern.clone())
                .push(")");
        }

        if let Some(faculty_id) = filter.faculty_id {
            query.push(" AND faculty_id = ").push_bind(faculty_id);
        }

        let departments = query
            .build_query_as::<Department>()
            .fetch_all(self.database.pool())
            .await?;

        Ok(departments)
    }

    pub async fn find_by_id(&self, id: &DepartmentId) -> AppResult<Option<Department>> {
        let item = sqlx::query_as::<_, Department>(
            "SELECT id, name, faculty_id FROM departments WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(self.database.pool())
        .await?;

        Ok(item)
    }

    pub async fn save(&self, department: &Department) -> AppResult<()> {
        sqlx::query("INSERT INTO departments (id, name, faculty_id) VALUES ($1, $2, $3)")
            .bind(department.id)
            .bind(&department.name)
            .bind(department.faculty_id)
            .execute(self.database.pool())
            .await?;

        Ok(())
    }
}
