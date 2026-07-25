use crate::auth::UserView;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginDto {
	#[validate(length(
		min = 1,
		max = 255,
		message = "El correo electrónico es obligatorio y debe tener entre 1 y 255 caracteres."
	))]
	pub email: String,

	#[validate(length(
		min = 1,
		max = 255,
		message = "La contraseña es obligatoria y debe tener entre 1 y 255 caracteres."
	))]
	pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
	pub user: UserView,
	pub access_token: String,
	pub access_token_exp: Timestamp,
	pub refresh_token_exp: Timestamp,
	pub refresh_token: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshResponse {
	pub access_token: String,
	pub access_token_exp: Timestamp,
}
