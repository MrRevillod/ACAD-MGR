use crate::research::classification::{ResearchFieldId, ResearchSubfield, ResearchSubfieldsRepository};

use std::sync::Arc;
use sword::prelude::*;

use crate::shared::AppResult;

#[injectable]
pub struct ResearchSubfieldsService {
    subfields: Arc<ResearchSubfieldsRepository>,
}

impl ResearchSubfieldsService {
    pub async fn list(&self, field_id: Option<ResearchFieldId>) -> AppResult<Vec<ResearchSubfield>> {
        self.subfields.list(field_id).await
    }
}
