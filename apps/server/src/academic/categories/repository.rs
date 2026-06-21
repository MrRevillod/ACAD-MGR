use crate::academic::{AcademicCategory, AcademicCategoryId, AcademicPlanta};
use crate::shared::{AppResult, Database, Tx};

use sqlx::QueryBuilder;
use std::sync::Arc;
use sword::prelude::*;

#[derive(Debug)]
pub struct AcademicCategoryFilter {
    pub name: Option<String>,
    pub planta: Option<AcademicPlanta>,
}

#[injectable]
pub struct AcademicCategoriesRepository {
    database: Arc<Database>,
}

impl AcademicCategoriesRepository {
    pub async fn list(&self, filter: AcademicCategoryFilter) -> AppResult<Vec<AcademicCategory>> {
        let mut query =
            QueryBuilder::new("SELECT id, name, planta FROM academic_categories WHERE 1=1");

        if let Some(n) = filter.name {
            let pattern = format!("%{}%", n.trim());
            query.push(" AND name ILIKE ").push_bind(pattern);
        }

        if let Some(planta) = filter.planta {
            query.push(" AND planta = ").push_bind(planta);
        }

        let items = query
            .build_query_as::<AcademicCategory>()
            .fetch_all(self.database.pool())
            .await?;

        Ok(items)
    }

    pub async fn find_by_id(&self, id: &AcademicCategoryId) -> AppResult<Option<AcademicCategory>> {
        let item = sqlx::query_as::<_, AcademicCategory>(
            "SELECT id, name, planta FROM academic_categories WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(self.database.pool())
        .await?;

        Ok(item)
    }

    pub async fn save(&self, category: &AcademicCategory) -> AppResult<()> {
        sqlx::query("INSERT INTO academic_categories (id, name, planta) VALUES ($1, $2, $3)")
            .bind(category.id)
            .bind(&category.name)
            .bind(&category.planta)
            .execute(self.database.pool())
            .await?;

        Ok(())
    }

    pub async fn save_tx(&self, tx: &mut Tx<'_>, category: &AcademicCategory) -> AppResult<()> {
        sqlx::query("INSERT INTO academic_categories (id, name, planta) VALUES ($1, $2, $3)")
            .bind(category.id)
            .bind(&category.name)
            .bind(&category.planta)
            .execute(&mut **tx)
            .await?;

        Ok(())
    }
}
