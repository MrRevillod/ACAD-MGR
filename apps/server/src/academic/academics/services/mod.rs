mod imports;

pub use imports::*;

use crate::{academic::*, shared::AppResult, university::*};
use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct AcademicsService {
    academics: Arc<AcademicsRepository>,
    departments: Arc<DepartmentsRepository>,
    careers: Arc<CareersRepository>,
    work_positions: Arc<AcademicWorkPositionsRepository>,
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
        let mut academic = Academic::from(input.clone());

        if self.academics.find_by_rut(&academic.rut).await?.is_some() {
            return Err(AcademicError::AcademicRutAlreadyExists)?;
        }

        if let Some(orcid) = &input.orcid
            && self.academics.find_by_orcid(orcid).await?.is_some()
        {
            return Err(AcademicError::AcademicOrcidAlreadyExists)?;
        }

        if self
            .departments
            .find_by_id(&academic.department_id)
            .await?
            .is_none()
        {
            return Err(UniversityError::DepartmentNotFound)?;
        }

        if let Some(career_id) = academic.career_id
            && self.careers.find_by_id(&career_id).await?.is_none()
        {
            return Err(UniversityError::CareerNotFound)?;
        }

        let work_position_id = match &input.work_position()? {
            WorkPositionResult::Id(id) => {
                if self.work_positions.find_by_id(id).await?.is_none() {
                    return Err(UniversityError::WorkPositionNotFound)?;
                }

                *id
            }
            WorkPositionResult::New(value) => {
                let work_position = AcademicWorkPosition::new(value.clone());
                match self.work_positions.save(&work_position).await {
                    Ok(_) => work_position.id,
                    Err(_) => self.work_positions.find_uknown().await?.id,
                }
            }
        };

        academic.work_position_id = work_position_id;

        self.academics.save(&academic).await?;
        self.find_view_by_id(&academic.id).await
    }
}
