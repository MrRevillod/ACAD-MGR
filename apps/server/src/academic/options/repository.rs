use crate::academic::*;
use crate::shared::{AppResult, Database};

use sqlx::{Postgres, QueryBuilder};
use std::sync::Arc;
use sword::prelude::*;

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
            "SELECT id, category_id, option, hours FROM academic_category_options WHERE 1=1",
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

    pub async fn find_one(
        &self,
        filter: AcademicCategoryOptionFilter,
    ) -> AppResult<Option<AcademicCategoryOption>> {
        let base = if filter.category_name.is_some() {
            "SELECT aco.id, aco.category_id, aco.option, aco.hours \
             FROM academic_category_options aco \
             JOIN academic_categories ac ON ac.id = aco.category_id \
             WHERE 1=1"
        } else {
            "SELECT aco.id, aco.category_id, aco.option, aco.hours \
             FROM academic_category_options aco WHERE 1=1"
        };

        let mut query = QueryBuilder::<Postgres>::new(base);

        if let Some(cid) = filter.category_id {
            query.push(" AND aco.category_id = ").push_bind(cid);
        }

        if let Some(option) = filter.option {
            query.push(" AND aco.option = ").push_bind(option);
        }

        if let Some(name) = filter.category_name {
            query.push(" AND ac.name = ").push_bind(name);
        }

        let item = query
            .build_query_as::<AcademicCategoryOption>()
            .fetch_optional(self.database.pool())
            .await?;

        Ok(item)
    }

    pub async fn find_by_id(
        &self,
        id: &AcademicCategoryOptionId,
    ) -> AppResult<Option<AcademicCategoryOption>> {
        let item = sqlx::query_as::<_, AcademicCategoryOption>(
            "SELECT id, category_id, option, hours FROM academic_category_options WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(self.database.pool())
        .await?;

        Ok(item)
    }

    pub async fn save(&self, option: &AcademicCategoryOption) -> AppResult<()> {
        sqlx::query(
            "INSERT INTO academic_category_options (id, category_id, option, hours) VALUES ($1, $2, $3, $4)",
        )
        .bind(option.id)
        .bind(option.category_id)
        .bind(option.option)
        .bind(option.hours)
        .execute(self.database.pool())
        .await?;

        Ok(())
    }
}
