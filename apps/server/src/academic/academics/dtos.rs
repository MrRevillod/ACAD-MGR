use std::sync::LazyLock;

use crate::academic::{Academic, AcademicCategoryOptionId, Sex};
use crate::shared::AppResult;
use crate::university::{AcademicWorkPositionId, CareerId, DepartmentId, UniversityError};

use chrono::{NaiveDate, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

static RUT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\d{7,8}-[\dkK]$").expect("regex inválida"));

static ORCID_ID_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\d{4}-\d{4}-\d{4}-\d{3}[\dX]$").expect("regex inválida"));

static UCT_FOUNDATION_DATE: NaiveDate = NaiveDate::from_ymd_opt(1959, 9, 8).unwrap();

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
    pub orcid: String,
    pub sex: Sex,

    #[validate(custom(function = "validate_birth_date"))]
    pub birth_date: NaiveDate,

    #[validate(custom(function = "validate_joined_at"))]
    pub joined_at: NaiveDate,
    pub work_position_id: Option<AcademicWorkPositionId>,
    pub work_position_new: Option<String>,
    pub work_position_details: Option<String>,
    pub department_id: DepartmentId,
    pub career_id: Option<CareerId>,
    pub uct_working_hours: f64,
    pub acad_category_options_id: AcademicCategoryOptionId,
    pub acad_category_hours: f64,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AcademicSortField {
    Names,
    PaternalSurname,
    MaternalSurname,
    JoinedAt,
    BirthDate,
}

#[derive(Debug, Serialize, Deserialize, Validate, Default)]
pub struct GetAcademicsQuery {
    pub search: Option<String>,
    pub sort: Option<AcademicSortField>,
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

fn validate_birth_date(date: &NaiveDate) -> Result<(), ValidationError> {
    validate_future_date(date)
}

fn validate_joined_at(joined_at: &NaiveDate) -> Result<(), ValidationError> {
    validate_future_date(joined_at)?;

    if *joined_at < UCT_FOUNDATION_DATE {
        return Err(ValidationError::new(
            "La fecha de ingreso no puede ser anterior al año de fundación de la universidad (1959)",
        ));
    };

    Ok(())
}

fn validate_future_date(date: &NaiveDate) -> Result<(), ValidationError> {
    if *date > Utc::now().naive_utc().date() {
        Err(ValidationError::new(
            "La fecha de nacimiento no puede ser en el futuro",
        ))
    } else {
        Ok(())
    }
}

impl From<CreateAcademicDto> for Academic {
    fn from(input: CreateAcademicDto) -> Self {
        Academic::builder()
            .rut(input.rut)
            .names(input.names)
            .paternal_surname(input.paternal_surname)
            .maternal_surname(input.maternal_surname)
            .email(input.email)
            .orcid(input.orcid)
            .sex(input.sex)
            .birth_date(input.birth_date)
            .joined_at(input.joined_at)
            .maybe_work_position_details(input.work_position_details)
            .department_id(input.department_id)
            .maybe_career_id(input.career_id)
            .uct_working_hours(input.uct_working_hours)
            .acad_category_options_id(input.acad_category_options_id)
            .acad_category_hours(input.acad_category_hours)
            .annual_discount_hours(input.annual_discount_hours)
            .nationality_code(input.nationality_code)
            .city(input.city)
            .build()
    }
}
