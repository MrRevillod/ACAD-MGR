use crate::{shared::AppResult, university::*};

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct AcademicWorkPositionsService {
	positions: Arc<AcademicWorkPositionsRepository>,
}

impl AcademicWorkPositionsService {
	pub async fn find(&self) -> AppResult<Vec<AcademicWorkPosition>> {
		self.positions.list().await
	}

	pub async fn create(
		&self,
		input: CreateAcademicWorkPositionDto,
	) -> AppResult<AcademicWorkPosition> {
		let position = AcademicWorkPosition::builder().name(input.name).build();

		self.positions.save(&position).await?;

		Ok(position)
	}
}
