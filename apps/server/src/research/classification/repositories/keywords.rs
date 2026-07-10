use crate::research::classification::{ResearchKeyword, ResearchKeywordId};
use crate::shared::{AppResult, Database};

use sqlx::Row;
use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct ResearchKeywordsRepository {
    database: Arc<Database>,
}

impl ResearchKeywordsRepository {
    pub async fn list(&self) -> AppResult<Vec<ResearchKeyword>> {
        sqlx::query_as::<_, ResearchKeyword>(
            "SELECT id, openalex_id, name FROM keywords ORDER BY name",
        )
        .fetch_all(self.database.pool())
        .await
        .map_err(Into::into)
    }

    pub async fn upsert(
        &self,
        openalex_id: &str,
        name: &str,
    ) -> AppResult<ResearchKeywordId> {
        let row = sqlx::query(
            "INSERT INTO keywords (openalex_id, name) VALUES ($1, $2) \
             ON CONFLICT (openalex_id) DO UPDATE SET name = EXCLUDED.name \
             RETURNING id",
        )
        .bind(openalex_id)
        .bind(name)
        .fetch_one(self.database.pool())
        .await?;
        Ok(ResearchKeywordId::from_uuid(row.get("id")))
    }

    pub async fn unknown_keyword_id(&self) -> AppResult<ResearchKeywordId> {
        let keyword: ResearchKeyword = sqlx::query_as(
            "SELECT id, openalex_id, name FROM keywords WHERE openalex_id = 'unknown'",
        )
        .fetch_one(self.database.pool())
        .await?;
        Ok(keyword.id)
    }
}
