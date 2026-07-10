mod imports;

pub use imports::*;

use super::normalize_orcid;
use crate::{academic::*, shared::AppResult, university::*};
use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct AcademicsService {
    academics: Arc<AcademicsRepository>,
    departments: Arc<DepartmentsRepository>,
    careers: Arc<CareersRepository>,
    work_positions: Arc<AcademicWorkPositionsRepository>,
    countries: Arc<CountriesRepository>,
    category_options: Arc<AcademicCategoryOptionsRepository>,
}

impl AcademicsService {
    pub async fn find(&self, query: GetAcademicsQuery) -> AppResult<Vec<AcademicView>> {
        let filter = AcademicListFilter {
            search: query.search,
            sort: query.sort,
            career_id: query.career_id,
            category_id: query.category_id,
            department_id: query.department_id,
            option: query.option,
            planta: query.planta,
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
        let academic = Academic::from(input.clone());

        if self.academics.find_by_rut(&academic.rut).await?.is_some() {
            return Err(AcademicError::AcademicRutAlreadyExists)?;
        }

        let normalized_orcid = input.orcid.as_deref().map(normalize_orcid);
        if let Some(ref orcid) = normalized_orcid
            && self.academics.find_by_orcid(orcid).await?.is_some()
        {
            return Err(AcademicError::AcademicOrcidAlreadyExists)?;
        }

        let Some(depto) = self.departments.find_by_id(&academic.department_id).await? else {
            return Err(UniversityError::DepartmentNotFound)?;
        };

        if let Some(career_id) = academic.career_id {
            let Some(career) = self.careers.find_by_id(&career_id).await? else {
                return Err(UniversityError::CareerNotFound)?;
            };

            if career.department_id != depto.id {
                return Err(UniversityError::CareerDepartmentMismatch)?;
            }
        }

        self.academics.save(&academic).await?;
        self.find_view_by_id(&academic.id).await
    }

    pub async fn update(
        &self,
        id: &AcademicId,
        input: UpdateAcademicDto,
    ) -> AppResult<AcademicView> {
        let Some(mut academic) = self.academics.find_by_id(id).await? else {
            return Err(AcademicError::AcademicNotFound)?;
        };

        if let Some(names) = &input.names {
            academic.names = names.clone();
        }

        if let Some(paternal_surname) = &input.paternal_surname {
            academic.paternal_surname = paternal_surname.clone();
        }

        if let Some(maternal_surname) = &input.maternal_surname {
            academic.maternal_surname = maternal_surname.clone();
        }

        let normalized_orcid = input.orcid.as_deref().map(normalize_orcid);
        if let Some(ref orcid) = normalized_orcid {
            if self.academics.find_by_orcid(orcid).await?.is_some()
                && Some(orcid.as_str()) != academic.orcid.as_deref()
            {
                return Err(AcademicError::AcademicOrcidAlreadyExists)?;
            }
            academic.orcid = Some(orcid.clone());
        }

        if let Some(sex) = input.sex {
            academic.sex = sex;
        }

        if let Some(birth_date) = input.birth_date {
            academic.birth_date = birth_date;
        }

        if let Some(joined_at) = input.joined_at {
            academic.joined_at = joined_at;
        }

        if let Some(wp_id) = input.work_position_id {
            if self.work_positions.find_by_id(&wp_id).await?.is_none() {
                return Err(UniversityError::WorkPositionNotFound)?;
            }
            academic.work_position_id = wp_id;
        }

        if let Some(dept_id) = input.department_id {
            if self.departments.find_by_id(&dept_id).await?.is_none() {
                return Err(UniversityError::DepartmentNotFound)?;
            }

            academic.department_id = dept_id;

            if let Some(career_id) = input.career_id {
                let Some(career) = self.careers.find_by_id(&career_id).await? else {
                    return Err(UniversityError::CareerNotFound)?;
                };

                if career.department_id != dept_id {
                    return Err(UniversityError::CareerDepartmentMismatch)?;
                }

                academic.career_id = Some(career_id);
            } else {
                academic.career_id = None;
            }
        } else if let Some(career_id) = input.career_id {
            let Some(career) = self.careers.find_by_id(&career_id).await? else {
                return Err(UniversityError::CareerNotFound)?;
            };

            if career.department_id != academic.department_id {
                return Err(UniversityError::CareerDepartmentMismatch)?;
            }

            academic.career_id = Some(career_id);
        }

        if let Some(cat_opt_id) = input.acad_category_options_id {
            if self
                .category_options
                .find_by_id(&cat_opt_id)
                .await?
                .is_none()
            {
                return Err(AcademicError::CategoryOptionNotFound)?;
            }
            academic.acad_category_options_id = cat_opt_id;
        }

        if let Some(jce) = input.jce {
            academic.jce = jce;
        }

        if let Some(hours) = input.annual_discount_hours {
            academic.annual_discount_hours = hours;
        }

        if let Some(code) = &input.nationality_code {
            if self.countries.find_by_code(code).await?.is_none() {
                return Err(UniversityError::CountryNotFound(code.clone()))?;
            }

            academic.nationality_code = code.clone();
        }

        if let Some(city) = input.city {
            academic.city = city;
        }

        self.academics.save(&academic).await?;
        self.find_view_by_id(&academic.id).await
    }
}
