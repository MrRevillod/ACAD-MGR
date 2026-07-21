use crate::academic::*;
use crate::auth::SessionCheck;
use crate::research::SyncResultView;

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
	#[interceptor(SessionCheck)]
	pub async fn get_academics(&self, req: Request) -> WebResult<Vec<AcademicView>> {
		let query = req.query_validator::<GetAcademicsQuery>()?;
		let academics = self.academics.find(query.unwrap_or_default()).await?;

		Ok(academics)
	}

	#[get("/{id}")]
	#[interceptor(SessionCheck)]
	pub async fn get_academic_view(&self, req: Request) -> WebResult<AcademicView> {
		let id = req.param::<AcademicId>("id")?;
		let academic = self.academics.find_view_by_id(&id).await?;

		Ok(academic)
	}

	#[get("/public")]
	pub async fn get_public_academics(&self, req: Request) -> WebResult<Vec<AcademicPublicView>> {
		let query = req.query_validator::<GetAcademicsQuery>()?;
		let academics = self.academics.find(query.unwrap_or_default()).await?;

		Ok(academics
			.into_iter()
			.map(AcademicPublicView::from)
			.collect())
	}

	#[get("/public/{id}")]
	pub async fn get_public_academic_view(&self, req: Request) -> WebResult<AcademicPublicView> {
		let id = req.param::<AcademicId>("id")?;
		let academic = self.academics.find_view_by_id(&id).await?;

		Ok(AcademicPublicView::from(academic))
	}

	#[patch("/{id}")]
	#[interceptor(SessionCheck)]
	pub async fn update_academic(&self, req: Request) -> WebResult<AcademicView> {
		let id = req.param::<AcademicId>("id")?;
		let input = req.body_validator::<UpdateAcademicDto>()?;
		let academic = self.academics.update(&id, input).await?;

		Ok(academic)
	}

	#[post("/")]
	#[interceptor(SessionCheck)]
	pub async fn create_academic(&self, req: Request) -> WebResult<AcademicView> {
		let input = req.body_validator::<CreateAcademicDto>()?;
		let academic = self.academics.create(input).await?;

		Ok(academic)
	}

	#[post("/{id}/update-profile-request")]
	pub async fn update_academic_profile_request(&self, req: Request) -> WebResult<AcademicView> {
		let id = req.param::<AcademicId>("id")?;
		let academic = self.academics.update_profile_request(&id).await?;

		Ok(academic)
	}

	#[post("/import")]
	#[interceptor(SessionCheck)]
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

	#[post("/profile/update/validate")]
	pub async fn validate_profile_update_token(
		&self,
		req: Request,
	) -> WebResult<AcademicPublicView> {
		let dto = req.body_validator::<ValidateTokenDto>()?;
		let academic_id = self.academics.validate_one_time_token(&dto.token).await?;
		let academic = self.academics.find_view_by_id(&academic_id).await?;

		Ok(AcademicPublicView::from(academic))
	}

	#[post("/profile/update")]
	pub async fn update_profile_from_token(&self, req: Request) -> WebResult<AcademicView> {
		let dto = req.body_validator::<CombinedSelfUpdateDto>()?;

		let academic = self
			.academics
			.update_from_one_time_link(&dto.token, dto.data)
			.await?;

		Ok(academic)
	}

	#[post("/profile/update/sync-works")]
	pub async fn sync_works_by_token(&self, req: Request) -> WebResult<SyncResultView> {
		let dto = req.body::<SyncByTokenDto>()?;
		let result = self.academics.sync_works_by_token(&dto.token).await?;

		Ok(result)
	}
}
