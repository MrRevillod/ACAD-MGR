use crate::{academic::*, shared::AppResult};
use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct DegreesService {
    degrees: Arc<DegreesRepository>,
}

impl DegreesService {
    pub async fn find(&self, academic_id: &AcademicId) -> AppResult<Vec<Degree>> {
        self.degrees.list(academic_id).await
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
