use crate::auth::UserView;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub user: UserView,
    pub access_token: String,
    pub access_token_exp: DateTime<Utc>,
    pub refresh_token: String,
    pub refresh_token_exp: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshResponse {
    pub access_token: String,
    pub access_token_exp: DateTime<Utc>,
}
