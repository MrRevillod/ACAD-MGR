use crate::research::classification::{ResearchDomainId, ResearchField};
use crate::shared::{AppResult, Database};

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct ResearchFieldsRepository {
    database: Arc<Database>,
}

impl ResearchFieldsRepository {
    pub async fn list(&self, domain_id: Option<ResearchDomainId>) -> AppResult<Vec<ResearchField>> {
        if let Some(domain_id) = domain_id {
            sqlx::query_as::<_, ResearchField>(
                "SELECT id, openalex_id, name, domain_id FROM research_fields WHERE domain_id = $1 ORDER BY name",
            )
            .bind(domain_id)
            .fetch_all(self.database.pool())
            .await
            .map_err(Into::into)
        } else {
            sqlx::query_as::<_, ResearchField>(
                "SELECT id, openalex_id, name, domain_id FROM research_fields ORDER BY name",
            )
            .fetch_all(self.database.pool())
            .await
            .map_err(Into::into)
        }
    }

    #[allow(dead_code)]
    pub async fn find_by_openalex_id(&self, openalex_id: &str) -> AppResult<Option<ResearchField>> {
        sqlx::query_as::<_, ResearchField>(
            "SELECT id, openalex_id, name, domain_id FROM research_fields WHERE openalex_id = $1",
        )
        .bind(openalex_id)
        .fetch_optional(self.database.pool())
        .await
        .map_err(Into::into)
    }
}
