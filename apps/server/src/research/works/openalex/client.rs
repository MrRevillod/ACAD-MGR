use crate::shared::AppResult;

use papers_openalex::{ListParams, OpenAlexClient as OaClient};
use serde::Deserialize;
use sword::prelude::*;

const SELECT_FIELDS: &[&str] = &[
    "id",
    "doi",
    "title",
    "display_name",
    "publication_year",
    "publication_date",
    "language",
    "type",
    "primary_location",
    "authorships",
    "primary_topic",
    "topics",
    "keywords",
    "abstract_inverted_index",
    "updated_date",
    "created_date",
];

#[derive(Debug, Clone, Deserialize)]
#[config(key = "openalex")]
pub struct OpenAlexConfig {
    pub api_key: String,
}

#[injectable(provider)]
pub struct OpenAlexClient {
    inner: OaClient,
}

impl OpenAlexClient {
    pub fn new(config: OpenAlexConfig) -> Self {
        Self {
            inner: OaClient::with_api_key(&config.api_key),
        }
    }

    pub async fn list_works_by_orcid(&self, orcid: &str) -> AppResult<Vec<papers_openalex::Work>> {
        let params = ListParams::builder()
            .filter(format!("authorships.author.orcid:{}", orcid))
            .select(SELECT_FIELDS.join(","))
            .build();
        let response = self
            .inner
            .list_works(&params)
            .await
            .map_err(|e| crate::research::works::WorksError::OpenAlexError(e.to_string()))?;
        Ok(response.results)
    }
}
