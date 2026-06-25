use crate::academic::*;
use crate::auth::SessionCheck;

use std::env::temp_dir;
use std::sync::Arc;
use sword::prelude::*;
use sword::web::*;
use uuid::Uuid;

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
        let file_path = temp_dir().join(format!("upload_{}.csv", Uuid::new_v4()));

        while let Some(field) = multipart.next_field().await? {
            if field.name() == Some("file") {
                let bytes = field.bytes().await?;
                self.imports.save_temp_csv(&file_path, &bytes).await?;
            }
        }

        let result = self.imports.process(&file_path).await?;

        self.imports.delete_temp_csv(&file_path).await?;

        Ok(result)
    }
}
