use crate::academic::AcademicId;
use crate::research::{CollaborationGraph, CollaborationsQuery, CollaborationsService};
use std::sync::Arc;

use sword::prelude::*;
use sword::web::*;

#[controller(kind = ControllerKind::Web, path = "/collaborations")]
pub struct CollaborationsController {
	service: Arc<CollaborationsService>,
}

impl CollaborationsController {
	#[get("/{academic_id}")]
	pub async fn get_collaborations(&self, req: Request) -> WebResult<CollaborationGraph> {
		let academic_id = req.param::<AcademicId>("academic_id")?;
		let query = req
			.query_validator::<CollaborationsQuery>()?
			.unwrap_or_default();

		Ok(self.service.get_collaborations(academic_id, query).await?)
	}
}
