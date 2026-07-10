use crate::research::classification::{ResearchDomain, ResearchDomainsRepository};

use std::sync::Arc;
use sword::prelude::*;

use crate::shared::AppResult;

#[injectable]
pub struct ResearchDomainsService {
    domains: Arc<ResearchDomainsRepository>,
}

impl ResearchDomainsService {
    pub async fn list(&self) -> AppResult<Vec<ResearchDomain>> {
        self.domains.list().await
    }
}
