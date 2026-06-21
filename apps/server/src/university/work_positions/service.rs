use crate::{shared::AppResult, university::*};

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct AcademicWorkPositionsService {
    positions: Arc<AcademicWorkPositionsRepository>,
}

impl AcademicWorkPositionsService {
    pub async fn find(&self, query: GetWorkPositionsQuery) -> AppResult<Vec<AcademicWorkPosition>> {
        let filter = WorkPositionFilter { name: query.name };

        self.positions.list(filter).await
    }

    pub async fn find_by_id(&self, id: &AcademicWorkPositionId) -> AppResult<AcademicWorkPosition> {
        let Some(position) = self.positions.find_by_id(id).await? else {
            return Err(UniversityError::WorkPositionNotFound)?;
        };

        Ok(position)
    }

    pub async fn create(
        &self,
        input: CreateAcademicWorkPositionDto,
    ) -> AppResult<AcademicWorkPosition> {
        let position = AcademicWorkPosition::new(input.name);

        self.positions.save(&position).await?;

        Ok(position)
    }
}
