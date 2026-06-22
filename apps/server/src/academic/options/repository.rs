use crate::academic::*;
use crate::shared::{AppResult, Database, Tx};

use sqlx::{Postgres, QueryBuilder};
use std::sync::Arc;
use sword::prelude::*;

#[derive(Debug)]
pub struct AcademicCategoryOptionFilter {
    pub category_id: Option<AcademicCategoryId>,
}

#[injectable]
pub struct AcademicCategoryOptionsRepository {
    database: Arc<Database>,
}

impl AcademicCategoryOptionsRepository {
    pub async fn list(
        &self,
        filter: AcademicCategoryOptionFilter,
    ) -> AppResult<Vec<AcademicCategoryOption>> {
        let mut query = QueryBuilder::<Postgres>::new(
            "SELECT id, category_id, option FROM academic_category_options WHERE 1=1",
        );

        if let Some(cid) = filter.category_id {
            query.push(" AND category_id = ").push_bind(cid);
        }

        let items = query
            .build_query_as::<AcademicCategoryOption>()
            .fetch_all(self.database.pool())
            .await?;

        Ok(items)
    }

    pub async fn find_by_category(
        &self,
        category_name: &str,
        planta: AcademicPlanta,
        option: AcademicOption,
    ) -> AppResult<Option<AcademicCategoryOptionId>> {
        let item = sqlx::query_scalar::<_, AcademicCategoryOptionId>(
            r#"
            SELECT aco.id
            FROM academic_category_options aco
            JOIN academic_categories ac ON ac.id = aco.category_id
            WHERE ac.name = $1 AND ac.planta = $2 AND aco.option = $3
            "#,
        )
        .bind(category_name)
        .bind(planta)
        .bind(option)
        .fetch_optional(self.database.pool())
        .await?;

        Ok(item)
    }

    pub async fn find_by_id(
        &self,
        id: &AcademicCategoryOptionId,
    ) -> AppResult<Option<AcademicCategoryOption>> {
        let item = sqlx::query_as::<_, AcademicCategoryOption>(
            "SELECT id, category_id, option FROM academic_category_options WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(self.database.pool())
        .await?;

        Ok(item)
    }

    pub async fn save(&self, option: &AcademicCategoryOption) -> AppResult<()> {
        sqlx::query(
            "INSERT INTO academic_category_options (id, category_id, option) VALUES ($1, $2, $3)",
        )
        .bind(option.id)
        .bind(&option.category_id)
        .bind(option.option)
        .execute(self.database.pool())
        .await?;

        Ok(())
    }

    pub async fn save_tx(&self, tx: &mut Tx<'_>, option: &AcademicCategoryOption) -> AppResult<()> {
        sqlx::query(
            "INSERT INTO academic_category_options (id, category_id, option) VALUES ($1, $2, $3)",
        )
        .bind(option.id)
        .bind(&option.category_id)
        .bind(option.option)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}
