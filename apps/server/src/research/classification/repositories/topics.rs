use crate::research::classification::{ResearchSubfieldId, ResearchTopic, ResearchTopicId};
use crate::shared::{AppResult, Database};

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct ResearchTopicsRepository {
    database: Arc<Database>,
}

impl ResearchTopicsRepository {
    pub async fn list(
        &self,
        subfield_id: Option<ResearchSubfieldId>,
    ) -> AppResult<Vec<ResearchTopic>> {
        if let Some(subfield_id) = subfield_id {
            Ok(sqlx::query_as::<_, ResearchTopic>(
                "SELECT id, openalex_id, name, subfield_id FROM research_topics WHERE subfield_id = $1 ORDER BY name",
            )
            .bind(subfield_id)
            .fetch_all(self.database.pool())
            .await?)
        } else {
            Ok(sqlx::query_as::<_, ResearchTopic>(
                "SELECT id, openalex_id, name, subfield_id FROM research_topics ORDER BY name",
            )
            .fetch_all(self.database.pool())
            .await?)
        }
    }

    #[allow(dead_code)]
    pub async fn find_by_openalex_id(&self, openalex_id: &str) -> AppResult<Option<ResearchTopic>> {
        sqlx::query_as::<_, ResearchTopic>(
            "SELECT id, openalex_id, name, subfield_id FROM research_topics WHERE openalex_id = $1",
        )
        .bind(openalex_id)
        .fetch_optional(self.database.pool())
        .await
        .map_err(Into::into)
    }

    pub async fn unknown_topic_id(&self) -> AppResult<ResearchTopicId> {
        let topic: ResearchTopic = sqlx::query_as(
            "SELECT id, openalex_id, name, subfield_id FROM research_topics WHERE openalex_id = 'unknown'",
        )
        .fetch_one(self.database.pool())
        .await?;
        Ok(topic.id)
    }
}
