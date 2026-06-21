use crate::{academic::*, shared::AppResult};
use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct DegreesService {
    degrees: Arc<DegreesRepository>,
}

impl DegreesService {
    pub async fn find(&self, query: GetDegreesQuery) -> AppResult<Vec<Degree>> {
        self.degrees.list(query.academic_id).await
    }

    pub async fn find_by_id(&self, id: &DegreeId) -> AppResult<Degree> {
        let Some(degree) = self.degrees.find_by_id(id).await? else {
            return Err(AcademicError::DegreeNotFound)?;
        };

        Ok(degree)
    }

    pub async fn create(&self, input: CreateDegreeDto) -> AppResult<Degree> {
        let degree = Degree::builder()
            .academic_id(input.academic_id)
            .name(input.name)
            .university(input.university)
            .obtained_at(input.obtained_at)
            .kind(input.kind)
            .country_code(input.country_code)
            .build();

        self.degrees.save(&degree).await?;

        Ok(degree)
    }
}
