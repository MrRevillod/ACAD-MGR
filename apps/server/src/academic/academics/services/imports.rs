use crate::academic::*;
use crate::shared::{AppError, AppResult, TransactionManager, Tx};
use crate::university::*;

use chrono::{NaiveDate, Utc};
use std::path::PathBuf;
use std::sync::Arc;
use sword::prelude::*;
use validator::Validate;

#[injectable]
pub struct ImportsService {
    tx: Arc<TransactionManager>,
    academics: Arc<AcademicsRepository>,
    degrees: Arc<DegreesRepository>,
    departments: Arc<DepartmentsRepository>,
    careers: Arc<CareersRepository>,
    work_positions: Arc<AcademicWorkPositionsRepository>,
    category_options: Arc<AcademicCategoryOptionsRepository>,
    countries: Arc<CountriesRepository>,
    categories: Arc<AcademicCategoriesRepository>,
}

impl ImportsService {
    pub async fn process(&self, file_path: &PathBuf) -> AppResult<ImportResult> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .trim(csv::Trim::All)
            .from_path(file_path)?;

        let headers = reader.headers()?.clone();
        let mut imported = 0usize;
        let mut errors = Vec::new();

        for (row_idx, result) in reader.records().enumerate() {
            let row_num = row_idx + 2;

            let Ok(record) = result else {
                let err = result.err().unwrap();
                let row_err = ImportRowError {
                    row: row_num,
                    reasons: vec![format!("Error de lectura en la fila: {err}")],
                };

                errors.push(row_err);

                continue;
            };

            if record.get(0).is_none_or(|v| v.trim().is_empty()) {
                continue;
            }

            let deserialization_result =
                record.deserialize::<'_, AcademicImportRowDto>(Some(&headers));

            let Ok(input) = deserialization_result else {
                let err = deserialization_result.err().unwrap();
                let row_err = ImportRowError {
                    row: row_num,
                    reasons: vec![format!("Error de formato en la fila: {err}")],
                };

                errors.push(row_err);

                continue;
            };

            if let Err(validation_errors) = input.validate() {
                errors.push(ImportRowError::from_validation(row_num, &validation_errors));
                continue;
            }

            let mut tx = self.tx.begin().await?;

            match self.process_row(&input, &mut tx).await {
                Ok(()) => {
                    tx.commit().await?;
                    imported += 1;
                }
                Err(e) => {
                    tx.rollback().await?;
                    errors.push(ImportRowError {
                        row: row_num,
                        reasons: vec![e.to_string()],
                    });
                }
            }
        }

        Ok(ImportResult { imported, errors })
    }

    async fn process_row(&self, input: &AcademicImportRowDto, tx: &mut Tx<'_>) -> AppResult<()> {
        let Some(department) = self
            .departments
            .find_by_name(&input.department_name)
            .await?
        else {
            return Err(UniversityError::DepartmentNotFound)?;
        };

        let career_id = match &input.career_name {
            Some(name) if !name.trim().is_empty() => {
                self.careers.find_by_name(name).await?.map(|c| c.id)
            }
            _ => None,
        };

        let Some(work_position_id) = self
            .work_positions
            .find_by_name(&input.work_position_name)
            .await?
            .map(|wp| wp.id)
        else {
            return Err(UniversityError::WorkPositionNotFound)?;
        };

        let option = AcademicOption::from(&input.option);

        let category_option_filter = AcademicCategoryOptionFilter {
            category_name: Some(input.category_name.clone()),
            option: Some(option),
            ..Default::default()
        };

        let Some(category_option) = self
            .category_options
            .find_one(category_option_filter)
            .await?
        else {
            return Err(AcademicError::CategoryOptionNotFound)?;
        };

        if let Some(expected_hours) = category_option.hours
            && expected_hours != *input.acad_category_hours
        {
            return Err(AcademicError::CategoryOptionHoursMismatch)?;
        }

        let Some(category) = self
            .categories
            .find_by_id(&category_option.category_id)
            .await?
        else {
            return Err(AcademicError::CategoryNotFound)?;
        };

        if category.name != input.category_name {
            return Err(AcademicError::CategoryOptionCategoryMismatch)?;
        }

        if category.planta != AcademicPlanta::from(&input.planta) {
            return Err(AcademicError::CategoryPlantaMismatch)?;
        }

        let category_option_id = category_option.id;
        let nationality_code = input.nationality_country.code.clone();

        let Some(_) = self.countries.find_by_code(&nationality_code).await? else {
            return Err(UniversityError::CountryNotFound(nationality_code))?;
        };

        let orcid = input
            .orcid
            .as_deref()
            .filter(|o| !o.trim().is_empty() && *o != "-")
            .unwrap_or("0000-0000-0000-0000")
            .to_string();

        let academic = Academic::builder()
            .rut(input.rut.clone())
            .names(input.names.clone())
            .paternal_surname(input.paternal_surname.clone())
            .maternal_surname(input.maternal_surname.clone())
            .email(input.email.clone())
            .orcid(orcid)
            .sex(input.sex)
            .birth_date(input.birth_date)
            .joined_at(input.joined_at)
            .work_position_id(work_position_id)
            .department_id(department.id)
            .maybe_career_id(career_id)
            .jce(*input.jce)
            .acad_category_options_id(category_option_id)
            .annual_discount_hours(*input.annual_discount_hours)
            .nationality_code(nationality_code)
            .city(input.city.clone())
            .build();

        self.academics.save_tx(tx, &academic).await?;

        self.save_degree(
            tx,
            &academic.id,
            &input.degree_1_name,
            &input.degree_1_university,
            input.degree_1_date,
            input.degree_1_country.as_deref(),
        )
        .await?;

        self.save_degree(
            tx,
            &academic.id,
            &input.degree_2_name,
            &input.degree_2_university,
            input.degree_2_date,
            input.degree_2_country.as_deref(),
        )
        .await?;

        Ok(())
    }

    async fn save_degree(
        &self,
        tx: &mut Tx<'_>,
        academic_id: &AcademicId,
        name: &Option<String>,
        university: &Option<String>,
        obtained_at: Option<NaiveDate>,
        country_code: Option<&str>,
    ) -> AppResult<()> {
        let name = name.as_deref().unwrap_or("Desconocido").trim();
        if name.is_empty() {
            return Ok(());
        }

        let university = university
            .as_deref()
            .unwrap_or("Desconocida")
            .trim()
            .to_string();

        let obtained_at = obtained_at.unwrap_or_else(|| Utc::now().date_naive());
        let country_code = country_code.unwrap_or("CL");

        let degree = Degree::builder()
            .academic_id(*academic_id)
            .name(name.to_string())
            .university(university)
            .obtained_at(obtained_at)
            .kind(DegreeKind::Base)
            .country_code(country_code.to_string())
            .build();

        self.degrees.save_tx(tx, &degree).await?;

        Ok(())
    }

    pub async fn save_temp_csv(&self, path: &PathBuf, content: &[u8]) -> AppResult<()> {
        tokio::fs::write(&path, content)
            .await
            .map_err(AppError::from)?;

        Ok(())
    }

    pub async fn delete_temp_csv(&self, path: &PathBuf) -> AppResult<()> {
        tokio::fs::remove_file(&path)
            .await
            .map_err(AppError::from)?;

        Ok(())
    }
}
