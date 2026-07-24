use crate::auth::{SessionCheck, UsersRepository};
use crate::research::*;
use crate::shared::RequestExt;

use std::sync::Arc;
use sword::events::EventPublisher;
use sword::prelude::*;
use sword::web::*;
use uuid::Uuid;

#[controller(kind = ControllerKind::Web, path = "/works")]
pub struct WorksController {
	works: Arc<WorksService>,
	events: Arc<EventPublisher>,
	users_repo: Arc<UsersRepository>,
}

impl WorksController {
	#[post("/sync/{id}")]
	#[interceptor(SessionCheck)]
	pub async fn sync_works(&self, req: Request) -> WebResult<SyncResultView> {
		let academic_id = req.param::<Uuid>("id")?;
		Ok(self.works.sync_from_openalex(academic_id).await?)
	}

	#[post("/sync-all")]
	#[interceptor(SessionCheck)]
	pub async fn sync_all_works(&self, req: Request) -> WebResult<JsonResponse> {
		let claims = req.claims().ok_or_else(JsonResponse::Unauthorized)?;
		let user = self
			.users_repo
			.find_by_id(&claims.user_id)
			.await?
			.ok_or_else(JsonResponse::Unauthorized)?;

		self.events
			.publish(SyncWorksRequest {
				user_email: user.email,
			})
			.await;

		Ok(JsonResponse::Accepted().message("Sincronización iniciada"))
	}

	#[get("/")]
	pub async fn list_works(&self, req: Request) -> WebResult<Vec<Work>> {
		let query = req.query_validator::<GetWorksQuery>()?;
		Ok(self.works.list(&query.unwrap_or_default()).await?)
	}

	#[get("/academic/{id}")]
	pub async fn list_works_by_academic(&self, req: Request) -> WebResult<Vec<Work>> {
		let academic_id = req.param::<Uuid>("id")?;
		let mut query = req.query_validator::<GetWorksQuery>()?.unwrap_or_default();

		query.academic_id = Some(academic_id);

		Ok(self.works.list(&query).await?)
	}

	#[get("/{id}")]
	pub async fn get_work(&self, req: Request) -> WebResult<WorkDetailView> {
		let work_id = req.param::<WorkId>("id")?;
		let work = self.works.find_by_id(work_id).await?;

		Ok(work)
	}

	#[put("/{id}/overrides")]
	#[interceptor(SessionCheck)]
	pub async fn update_work_overrides(&self, req: Request) -> WebResult<JsonResponse> {
		let work_id = req.param::<WorkId>("id")?;
		let input = req.body_validator::<WorkOverridesInput>()?;
		self.works.update_overrides(work_id, input).await?;
		Ok(JsonResponse::Ok().message("Overrides actualizados"))
	}

	#[delete("/{id}/overrides")]
	#[interceptor(SessionCheck)]
	pub async fn clear_work_overrides(&self, req: Request) -> WebResult<JsonResponse> {
		let work_id = req.param::<WorkId>("id")?;
		self.works.clear_overrides(work_id).await?;
		Ok(JsonResponse::Ok().message("Overrides eliminados"))
	}
}
