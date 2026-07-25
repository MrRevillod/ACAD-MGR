use crate::shared::{AppError, AppResult, Database};
use crate::university::{AcademicWorkPosition, AcademicWorkPositionId};

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct AcademicWorkPositionsRepository {
	database: Arc<Database>,
}

impl AcademicWorkPositionsRepository {
	pub async fn list(&self) -> AppResult<Vec<AcademicWorkPosition>> {
		AcademicWorkPosition::all()
			.exec(&mut self.database.pool())
			.await?
			.map_err(AppError::from)
	}

	pub async fn find_by_id(
		&self,
		id: &AcademicWorkPositionId,
	) -> AppResult<Option<AcademicWorkPosition>> {
		AcademicWorkPosition::get_by_id(&mut self.database.pool(), id)
			.await?
			.map_err(AppError::from)
	}

	pub async fn save(&self, position: &AcademicWorkPosition) -> AppResult<()> {
		AcademicWorkPosition::create()
			.id(position.id)
			.name(position.name.clone())
			.exec(&mut self.database.pool())
			.await?
			.map_err(AppError::from)
	}
}
