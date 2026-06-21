use crate::{academic::*, shared::AppResult};
use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct AcademicsService {
    academics: Arc<AcademicsRepository>,
}

impl AcademicsService {
    pub async fn find(&self, query: GetAcademicsQuery) -> AppResult<Vec<AcademicView>> {
        let filter = AcademicListFilter {
            search: query.search,
            sort: query.sort,
        };

        self.academics.list(filter).await
    }

    pub async fn find_view_by_id(&self, id: &AcademicId) -> AppResult<AcademicView> {
        let Some(view) = self.academics.find_view_by_id(id).await? else {
            return Err(AcademicError::AcademicNotFound)?;
        };

        Ok(view)
    }

    pub async fn create(&self, input: CreateAcademicDto) -> AppResult<AcademicView> {
        let academic = Academic::from(input);

        self.academics.save(&academic).await?;
        self.find_view_by_id(&academic.id).await
    }
}
