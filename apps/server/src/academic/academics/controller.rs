use crate::academic::*;
use crate::auth::SessionCheck;
use crate::shared::AppError;

use std::env::temp_dir;
use std::sync::Arc;
use sword::prelude::*;
use sword::web::*;

#[controller(kind = ControllerKind::Web, path = "/academics")]
pub struct AcademicsController {
    academics: Arc<AcademicsService>,
    imports: Arc<ImportsService>,
}

impl AcademicsController {
    #[get("/")]
    pub async fn get_academics(&self, req: Request) -> WebResult<Vec<AcademicView>> {
        let query = req.query_validator::<GetAcademicsQuery>()?;
        let academics = self.academics.find(query.unwrap_or_default()).await?;

        Ok(academics)
    }

    #[get("/{id}")]
    pub async fn get_academic(&self, req: Request) -> WebResult<AcademicView> {
        let id = req.param::<AcademicId>("id")?;
        let academic = self.academics.find_view_by_id(&id).await?;

        Ok(academic)
    }

    #[post("/")]
    #[interceptor(SessionCheck)]
    pub async fn create_academic(&self, req: Request) -> WebResult<AcademicView> {
        let input = req.body_validator::<CreateAcademicDto>()?;
        let academic = self.academics.create(input).await?;

        Ok(academic)
    }

    #[post("/import")]
    pub async fn import_academics(&self, req: Request) -> WebResult<ImportResult> {
        let mut multipart = req.multipart().await?;
        let file_path = temp_dir().join(format!("upload_{}.csv", uuid::Uuid::new_v4()));

        while let Some(field) = multipart.next_field().await? {
            if field.name() == Some("file") {
                tokio::fs::write(&file_path, field.bytes().await?)
                    .await
                    .map_err(AppError::from)?
            }
        }

        let result = self.imports.process(file_path.to_str().unwrap()).await?;

        Ok(result)
    }
}
