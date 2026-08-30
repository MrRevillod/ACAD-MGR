use super::SelfUpdateAcademicDto;
use crate::research::WorkOverridesInput;

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ValidateTokenDto {
	#[validate(length(min = 1, message = "El token es obligatorio"))]
	pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ProfileUpdateRequestDto {
	#[validate(length(min = 1, message = "El código de autorización es obligatorio"))]
	pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncByTokenDto {
	pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CombinedSelfUpdateDto {
	#[validate(length(min = 1, message = "El token es obligatorio"))]
	pub token: String,

	#[validate(nested)]
	pub data: SelfUpdateAcademicDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct WorkOverridesByTokenDto {
	#[validate(length(min = 1, message = "El token es obligatorio"))]
	pub token: String,

	#[validate(nested)]
	pub data: WorkOverridesInput,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AffiliationsByTokenDto {
	#[validate(length(min = 1, message = "El token es obligatorio"))]
	pub token: String,

	pub affiliations: Vec<String>,
}
