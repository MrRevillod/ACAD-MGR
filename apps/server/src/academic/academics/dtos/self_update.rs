use super::ORCID_ID_REGEX;
use crate::academic::Sex;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SelfUpdateAcademicDto {
	#[validate(length(
		min = 1,
		max = 255,
		message = "Los nombres deben tener entre 1 y 255 caracteres"
	))]
	pub names: Option<String>,

	#[validate(length(
		min = 1,
		max = 255,
		message = "El apellido paterno debe tener entre 1 y 255 caracteres"
	))]
	pub paternal_surname: Option<String>,

	#[validate(length(
		min = 1,
		max = 255,
		message = "El apellido materno debe tener entre 1 y 255 caracteres"
	))]
	pub maternal_surname: Option<String>,

	#[validate(regex(
		path = *ORCID_ID_REGEX,
		message = "El ORCID ID debe tener el formato XXXX-XXXX-XXXX-XXXX"
	))]
	pub orcid: Option<String>,
	pub sex: Option<Sex>,

	#[validate(custom(function = "super::validate_birth_date"))]
	pub birth_date: Option<NaiveDate>,

	#[validate(length(
		min = 2,
		max = 2,
		message = "El código de país debe tener 2 caracteres"
	))]
	pub nationality_code: Option<String>,

	#[validate(length(
		min = 1,
		max = 255,
		message = "La ciudad debe tener entre 1 y 255 caracteres"
	))]
	pub city: Option<String>,
}
