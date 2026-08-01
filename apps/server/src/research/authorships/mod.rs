mod repository;
pub use repository::AuthorshipsRepository;

use crate::research::WorkId;
use bon::Builder;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Type, Serialize, Deserialize, Eq, PartialEq)]
#[sqlx(type_name = "authorship_position", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum AuthorshipPosition {
	First,
	Middle,
	Last,
}

#[derive(Debug, Serialize, FromRow, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Authorship {
	pub work_id: WorkId,
	pub orcid: String,
	pub name: String,
	pub is_external: bool,
	pub is_corresponding: bool,
	pub affiliations: Vec<String>,
	pub position: AuthorshipPosition,
	pub academic_id: Option<Uuid>,
}
