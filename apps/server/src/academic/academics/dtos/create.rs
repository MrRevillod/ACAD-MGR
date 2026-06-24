use crate::academic::{Academic, AcademicCategoryOptionId, Sex};
use crate::shared::AppResult;
use crate::university::{AcademicWorkPositionId, CareerId, DepartmentId, UniversityError};

use super::{ORCID_ID_REGEX, RUT_REGEX};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
#[validate(schema(function = "validate_create_academic_dto"))]
pub struct CreateAcademicDto {
    #[validate(regex(
		path = *RUT_REGEX,
		message = "El RUT debe tener el formato XXXXXXXX-X"
	))]
    pub rut: String,

    #[validate(length(
        min = 1,
        max = 255,
        message = "Los nombres deben tener entre 1 y 255 caracteres"
    ))]
    pub names: String,

    #[validate(length(
        min = 1,
        max = 255,
        message = "El apellido paterno debe tener entre 1 y 255 caracteres"
    ))]
    pub paternal_surname: String,

    #[validate(length(
        min = 1,
        max = 255,
        message = "El apellido materno debe tener entre 1 y 255 caracteres"
    ))]
    pub maternal_surname: String,

    #[validate(email(message = "El email debe ser válido"))]
    pub email: String,

    #[validate(regex(
		path = *ORCID_ID_REGEX,
		message = "El ORCID ID debe tener el formato XXXX-XXXX-XXXX-XXXX"
	))]
    pub orcid: Option<String>,
    pub sex: Sex,

    #[validate(custom(function = "super::validate_birth_date"))]
    pub birth_date: NaiveDate,

    #[validate(custom(function = "super::validate_joined_at"))]
    pub joined_at: NaiveDate,
    pub work_position_id: Option<AcademicWorkPositionId>,
    pub work_position_new: Option<String>,
    pub department_id: DepartmentId,
    pub career_id: Option<CareerId>,
    pub acad_category_options_id: AcademicCategoryOptionId,

    #[validate(range(
        min = 0.0,
        max = 1.0,
        message = "Las horas de trabajo en la universidad no pueden ser negativas"
    ))]
    pub jce: f64,

    #[validate(range(
        min = 0.0,
        message = "Las horas de categoría académica no pueden ser negativas"
    ))]
    pub acad_category_hours: f64,

    #[validate(range(
        min = 0.0,
        message = "Las horas de descuento anual no pueden ser negativas"
    ))]
    pub annual_discount_hours: f64,

    #[validate(length(
        min = 2,
        max = 2,
        message = "El código de país debe tener 2 caracteres"
    ))]
    pub nationality_code: String,

    #[validate(length(
        min = 1,
        max = 255,
        message = "La ciudad debe tener entre 1 y 255 caracteres"
    ))]
    pub city: String,
}

pub enum WorkPositionResult {
    Id(AcademicWorkPositionId),
    New(String),
}

impl CreateAcademicDto {
    pub fn work_position(&self) -> AppResult<WorkPositionResult> {
        if let Some(id) = self.work_position_id {
            Ok(WorkPositionResult::Id(id))
        } else if let Some(name) = &self.work_position_new {
            Ok(WorkPositionResult::New(name.clone()))
        } else {
            Err(UniversityError::WorkPositionMissing)?
        }
    }
}

fn validate_create_academic_dto(dto: &CreateAcademicDto) -> Result<(), ValidationError> {
    match (&dto.work_position_id, &dto.work_position_new) {
        (Some(_), Some(_)) => {
            return Err(ValidationError::new(
                "No se puede proporcionar un work_position_id y un work_position_new al mismo tiempo",
            ));
        }
        (None, None) => {
            return Err(ValidationError::new(
                "Se debe proporcionar un work_position_id o un work_position_new",
            ));
        }
        _ => {}
    }

    Ok(())
}

impl From<CreateAcademicDto> for Academic {
    fn from(input: CreateAcademicDto) -> Self {
        Academic::builder()
            .rut(input.rut)
            .names(input.names)
            .paternal_surname(input.paternal_surname)
            .maternal_surname(input.maternal_surname)
            .email(input.email)
            .maybe_orcid(input.orcid)
            .sex(input.sex)
            .jce(input.jce)
            .birth_date(input.birth_date)
            .joined_at(input.joined_at)
            .department_id(input.department_id)
            .maybe_career_id(input.career_id)
            .acad_category_options_id(input.acad_category_options_id)
            .acad_category_hours(input.acad_category_hours)
            .annual_discount_hours(input.annual_discount_hours)
            .nationality_code(input.nationality_code)
            .city(input.city)
            .build()
    }
}
