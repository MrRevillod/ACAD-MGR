use crate::research::classification::{ResearchDomainId, ResearchField, ResearchFieldsRepository};

use std::sync::Arc;
use sword::prelude::*;

use crate::shared::AppResult;

#[injectable]
pub struct ResearchFieldsService {
    fields: Arc<ResearchFieldsRepository>,
}

impl ResearchFieldsService {
    pub async fn list(&self, domain_id: Option<ResearchDomainId>) -> AppResult<Vec<ResearchField>> {
        self.fields.list(domain_id).await
    }
}
