use crate::{shared::AppResult, university::*};

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct AcademicWorkPositionsService {
    positions: Arc<AcademicWorkPositionsRepository>,
}

impl AcademicWorkPositionsService {
    pub async fn find(&self, query: GetWorkPositionsQuery) -> AppResult<Vec<AcademicWorkPosition>> {
        let filter = WorkPositionFilter {
            name: query.name,
            code: query.code,
        };

        self.positions.list(filter).await
    }

    pub async fn find_by_code(&self, code: &str) -> AppResult<AcademicWorkPosition> {
        let Some(position) = self.positions.find_by_code(code).await? else {
            return Err(UniversityError::WorkPositionNotFound)?;
        };

        Ok(position)
    }

    pub async fn create(
        &self,
        input: CreateAcademicWorkPositionDto,
    ) -> AppResult<AcademicWorkPosition> {
        let position = AcademicWorkPosition {
            code: input.code,
            name: input.name,
        };

        self.positions.save(&position).await?;

        Ok(position)
    }
}
