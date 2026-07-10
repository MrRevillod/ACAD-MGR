use crate::research::classification::*;

use std::sync::Arc;
use sword::prelude::*;
use sword::web::*;

#[controller(kind = ControllerKind::Web, path = "/works-classification")]
pub struct WorksClassificationController {
    domains: Arc<ResearchDomainsService>,
    fields: Arc<ResearchFieldsService>,
    subfields: Arc<ResearchSubfieldsService>,
    topics: Arc<ResearchTopicsService>,
    keywords: Arc<ResearchKeywordsService>,
}

impl WorksClassificationController {
    #[get("/domains")]
    pub async fn get_domains(&self, _req: Request) -> WebResult<Vec<ResearchDomain>> {
        Ok(self.domains.list().await?)
    }

    #[get("/fields")]
    pub async fn get_fields(&self, req: Request) -> WebResult<Vec<ResearchField>> {
        let query = req.query_validator::<GetFieldsQuery>()?;
        let domain_id = query
            .unwrap_or_default()
            .domain_id
            .map(ResearchDomainId::from_uuid);
        Ok(self.fields.list(domain_id).await?)
    }

    #[get("/subfields")]
    pub async fn get_subfields(&self, req: Request) -> WebResult<Vec<ResearchSubfield>> {
        let query = req.query_validator::<GetSubfieldsQuery>()?;
        let field_id = query
            .unwrap_or_default()
            .field_id
            .map(ResearchFieldId::from_uuid);
        Ok(self.subfields.list(field_id).await?)
    }

    #[get("/topics")]
    pub async fn get_topics(&self, req: Request) -> WebResult<Vec<ResearchTopic>> {
        let query = req.query_validator::<GetTopicsQuery>()?;
        let subfield_id = query
            .unwrap_or_default()
            .subfield_id
            .map(ResearchSubfieldId::from_uuid);
        Ok(self.topics.list(subfield_id).await?)
    }

    #[get("/keywords")]
    pub async fn get_keywords(&self, _req: Request) -> WebResult<Vec<ResearchKeyword>> {
        Ok(self.keywords.list().await?)
    }
}
