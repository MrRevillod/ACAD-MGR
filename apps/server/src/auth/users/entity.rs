use crate::model_id;
use serde::{Deserialize, Serialize};
use toasty::{Embed, Model};

model_id! {
	struct UserId, key: "user"
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, Embed)]
#[serde(rename_all = "lowercase")]
#[column(rename_all = "lowercase")]
pub enum UserRole {
	Admin,
}

#[derive(Debug, Clone, Model)]
pub struct User {
	#[key]
	pub id: UserId,

	#[unique]
	pub email: String,

	pub name: String,
	pub role: UserRole,
	pub password_hash: String,
}

#[derive(Debug)]
pub struct UserFilter {
	pub search: Option<String>,
	pub role: Option<UserRole>,
}
