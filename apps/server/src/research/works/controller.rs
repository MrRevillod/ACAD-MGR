use crate::auth::SessionCheck;
use crate::research::works::dtos::GetWorksQuery;
use crate::research::works::errors::WorksError;
use crate::research::works::service::WorksService;
use crate::research::works::views::{SyncResultView, WorkDetailView, WorkView};

use std::sync::Arc;
use sword::prelude::*;
use sword::web::*;
use uuid::Uuid;

#[controller(kind = ControllerKind::Web, path = "/works")]
pub struct WorksController {
    works: Arc<WorksService>,
}

impl WorksController {
    #[post("/sync/{id}")]
    #[interceptor(SessionCheck)]
    pub async fn sync_works(&self, req: Request) -> WebResult<SyncResultView> {
        let academic_id = req.param::<Uuid>("id")?;
        Ok(self.works.sync_from_openalex(academic_id).await?)
    }

    #[get("/")]
    pub async fn list_works(&self, req: Request) -> WebResult<Vec<WorkView>> {
        let query = req.query_validator::<GetWorksQuery>()?;
        Ok(self.works.list(&query.unwrap_or_default()).await?)
    }

    #[get("/academic/{id}")]
    pub async fn list_works_by_academic(&self, req: Request) -> WebResult<Vec<WorkView>> {
        let academic_id = req.param::<Uuid>("id")?;
        let query = GetWorksQuery {
            academic_id: Some(academic_id),
            ..Default::default()
        };
        Ok(self.works.list(&query).await?)
    }

    #[get("/{id}")]
    pub async fn get_work(&self, req: Request) -> WebResult<WorkDetailView> {
        let work_id = req.param::<Uuid>("id")?;
        let work = self
            .works
            .find_by_id(crate::research::works::entity::WorkId::from_uuid(work_id))
            .await?
            .ok_or(WorksError::WorkNotFound)?;
        Ok(work)
    }
}
