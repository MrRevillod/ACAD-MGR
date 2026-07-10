use crate::research::classification::ResearchDomain;
use crate::shared::{AppResult, Database};

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct ResearchDomainsRepository {
    database: Arc<Database>,
}

impl ResearchDomainsRepository {
    pub async fn list(&self) -> AppResult<Vec<ResearchDomain>> {
        sqlx::query_as::<_, ResearchDomain>(
            "SELECT id, openalex_id, name FROM research_domains ORDER BY name",
        )
        .fetch_all(self.database.pool())
        .await
        .map_err(Into::into)
    }

    #[allow(dead_code)]
    pub async fn find_by_openalex_id(
        &self,
        openalex_id: &str,
    ) -> AppResult<Option<ResearchDomain>> {
        sqlx::query_as::<_, ResearchDomain>(
            "SELECT id, openalex_id, name FROM research_domains WHERE openalex_id = $1",
        )
        .bind(openalex_id)
        .fetch_optional(self.database.pool())
        .await
        .map_err(Into::into)
    }
}
