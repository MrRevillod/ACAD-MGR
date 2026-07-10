use crate::research::classification::{ResearchKeyword, ResearchKeywordsRepository};

use std::sync::Arc;
use sword::prelude::*;

use crate::shared::AppResult;

#[injectable]
pub struct ResearchKeywordsService {
    keywords: Arc<ResearchKeywordsRepository>,
}

impl ResearchKeywordsService {
    pub async fn list(&self) -> AppResult<Vec<ResearchKeyword>> {
        self.keywords.list().await
    }
}
