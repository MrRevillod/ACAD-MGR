use crate::research::classification::{ResearchFieldId, ResearchSubfield};
use crate::shared::{AppResult, Database};

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct ResearchSubfieldsRepository {
    database: Arc<Database>,
}

impl ResearchSubfieldsRepository {
    pub async fn list(
        &self,
        field_id: Option<ResearchFieldId>,
    ) -> AppResult<Vec<ResearchSubfield>> {
        if let Some(field_id) = field_id {
            sqlx::query_as::<_, ResearchSubfield>(
                "SELECT id, openalex_id, name, field_id FROM research_subfields WHERE field_id = $1 ORDER BY name",
            )
            .bind(field_id)
            .fetch_all(self.database.pool())
            .await
            .map_err(Into::into)
        } else {
            sqlx::query_as::<_, ResearchSubfield>(
                "SELECT id, openalex_id, name, field_id FROM research_subfields ORDER BY name",
            )
            .fetch_all(self.database.pool())
            .await
            .map_err(Into::into)
        }
    }

    #[allow(dead_code)]
    pub async fn find_by_openalex_id(
        &self,
        openalex_id: &str,
    ) -> AppResult<Option<ResearchSubfield>> {
        sqlx::query_as::<_, ResearchSubfield>(
            "SELECT id, openalex_id, name, field_id FROM research_subfields WHERE openalex_id = $1",
        )
        .bind(openalex_id)
        .fetch_optional(self.database.pool())
        .await
        .map_err(Into::into)
    }
}
