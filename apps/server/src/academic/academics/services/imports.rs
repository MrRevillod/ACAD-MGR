use crate::academic::*;
use crate::shared::{AppError, AppResult, Database, Tx};
use crate::university::*;

use chrono::{NaiveDate, Utc};
use std::sync::Arc;
use sword::prelude::*;
use validator::Validate;

#[injectable]
pub struct ImportsService {
    database: Arc<Database>,
    academics: Arc<AcademicsRepository>,
    degrees: Arc<DegreesRepository>,
    departments: Arc<DepartmentsRepository>,
    careers: Arc<CareersRepository>,
    work_positions: Arc<AcademicWorkPositionsRepository>,
    category_options: Arc<AcademicCategoryOptionsRepository>,
}

impl ImportsService {
    pub async fn process(&self, file_path: &str) -> AppResult<ImportResult> {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .trim(csv::Trim::All)
            .from_path(file_path)?;

        let headers = reader.headers()?.clone();
        let mut imported = 0usize;
        let mut errors = Vec::new();

        for (row_idx, result) in reader.records().enumerate() {
            let row_num = row_idx + 2;

            let record = match result {
                Ok(r) => r,
                Err(e) => {
                    errors.push(ImportRowError {
                        row: row_num,
                        reasons: vec![format!("Error de lectura en la fila: {e}")],
                    });
                    continue;
                }
            };

            if record.get(0).map_or(true, |v| v.trim().is_empty()) {
                continue;
            }

            let input: AcademicImportRowDto = match record.deserialize(Some(&headers)) {
                Ok(r) => r,
                Err(e) => {
                    errors.push(ImportRowError {
                        row: row_num,
                        reasons: vec![format!("Error de formato en la fila: {e}")],
                    });
                    continue;
                }
            };

            if let Err(validation_errors) = input.validate() {
                let reasons: Vec<String> = validation_errors
                    .field_errors()
                    .iter()
                    .flat_map(|(field, errs)| {
                        errs.iter().map(move |e| {
                            let msg = e
                                .message
                                .as_ref()
                                .map(|m| m.to_string())
                                .unwrap_or_default();
                            format!("{}: {}", field, msg)
                        })
                    })
                    .collect();

                errors.push(ImportRowError {
                    row: row_num,
                    reasons,
                });
                continue;
            }

            let mut tx = self.database.tx().await?;

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
        let department = self
            .departments
            .find_by_name(&input.department_name)
            .await?
            .ok_or_else(|| {
                let msg = format!("Departamento '{}' no encontrado", input.department_name);
                AppError::from(std::io::Error::new(std::io::ErrorKind::NotFound, msg))
            })?;

        let career_id = if let Some(ref name) = input.career_name {
            if !name.trim().is_empty() {
                self.careers.find_by_name(name).await?.map(|c| c.id)
            } else {
                None
            }
        } else {
            None
        };

        let work_position = self
            .work_positions
            .find_by_name(&input.work_position_name)
            .await?;

        let work_position_id = match work_position {
            Some(wp) => wp.id,
            None => self.work_positions.find_uknown().await?.id,
        };

        let planta: AcademicPlanta = input.planta.clone().into();
        let option: AcademicOption = input.option.clone().into();

        let category_option_id = self
            .category_options
            .find_by_category(&input.category_name, planta, option)
            .await?
            .ok_or_else(|| {
                let msg = format!(
                    "Opción de categoría no encontrada para categoría '{}'",
                    input.category_name
                );
                AppError::from(std::io::Error::new(std::io::ErrorKind::NotFound, msg))
            })?;

        let nationality_code = input
            .nationality_country
            .split(" - ")
            .next()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .ok_or_else(|| {
                let msg = format!("Código de país inválido: '{}'", input.nationality_country);
                AppError::from(std::io::Error::new(std::io::ErrorKind::InvalidData, msg))
            })?;

        let orcid = input
            .orcid
            .as_ref()
            .and_then(|o| {
                let t = o.trim();
                if t.is_empty() || t == "-" {
                    None
                } else {
                    Some(t.to_string())
                }
            })
            .unwrap_or_else(|| "0000-0000-0000-0000".to_string());

        let uct_working_hours = parse_f64(&input.uct_working_hours)
            .map_err(|e| AppError::from(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;

        let acad_category_hours = parse_f64(&input.acad_category_hours)
            .map_err(|e| AppError::from(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;

        let annual_discount_hours = parse_f64(&input.annual_discount_hours)
            .map_err(|e| AppError::from(std::io::Error::new(std::io::ErrorKind::InvalidData, e)))?;

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
            .maybe_work_position_details(input.work_position_details.clone())
            .department_id(department.id)
            .maybe_career_id(career_id)
            .uct_working_hours(uct_working_hours)
            .acad_category_options_id(category_option_id)
            .acad_category_hours(acad_category_hours)
            .annual_discount_hours(annual_discount_hours)
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
        )
        .await?;

        self.save_degree(
            tx,
            &academic.id,
            &input.degree_2_name,
            &input.degree_2_university,
            input.degree_2_date,
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

        let degree = Degree::builder()
            .academic_id(*academic_id)
            .name(name.to_string())
            .university(university)
            .obtained_at(obtained_at)
            .kind(DegreeKind::Base)
            .country_code("CL".to_string())
            .build();

        self.degrees.save_tx(tx, &degree).await?;

        Ok(())
    }
}

fn parse_f64(s: &str) -> Result<f64, String> {
    let s = s.trim().replace(",", ".");

    if s.is_empty() {
        return Err("valor numérico vacío".to_string());
    }

    s.parse::<f64>()
        .map_err(|_| format!("valor numérico inválido: '{}'", s))
}
