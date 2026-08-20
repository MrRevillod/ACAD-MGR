use crate::config::AppConfig;

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAppConfigDto {
	#[validate(range(min = 1.0, message = "El valor máximo de JCE debe ser mayor que 0"))]
	pub jce_max: f64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfigResponse {
	pub jce_max: f64,
}

impl From<AppConfig> for AppConfigResponse {
	fn from(value: AppConfig) -> Self {
		Self {
			jce_max: value.jce_max,
		}
	}
}
