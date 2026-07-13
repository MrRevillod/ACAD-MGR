use crate::research::works::WorkId;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

#[derive(Debug, Clone, Copy, Type, Serialize, Deserialize, Eq, PartialEq)]
#[sqlx(type_name = "authorship_position", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum AuthorshipPosition {
	First,
	Middle,
	Last,
}

#[derive(Debug, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Authorship {
	pub work_id: WorkId,
	pub orcid: String,
	pub name: String,
	pub is_external: bool,
	pub is_corresponding: bool,
	pub affiliations: Vec<String>,
	pub position: AuthorshipPosition,
}
