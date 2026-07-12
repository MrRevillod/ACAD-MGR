use crate::academic::AcademicPlanta;

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateAcademicCategoryDto {
	#[validate(length(
		min = 1,
		max = 255,
		message = "El nombre debe tener entre 1 y 255 caracteres"
	))]
	pub name: String,
	pub planta: AcademicPlanta,
}

#[derive(Debug, Serialize, Deserialize, Validate, Default)]
pub struct GetCategoriesQuery {
	#[validate(length(max = 255, message = "El nombre no puede tener más de 255 caracteres"))]
	pub name: Option<String>,
	pub planta: Option<AcademicPlanta>,
}
