use crate::research::classification::{ResearchSubfieldId, ResearchTopic, ResearchTopicsRepository};

use std::sync::Arc;
use sword::prelude::*;

use crate::shared::AppResult;

#[injectable]
pub struct ResearchTopicsService {
    topics: Arc<ResearchTopicsRepository>,
}

impl ResearchTopicsService {
    pub async fn list(&self, subfield_id: Option<ResearchSubfieldId>) -> AppResult<Vec<ResearchTopic>> {
        self.topics.list(subfield_id).await
    }
}
