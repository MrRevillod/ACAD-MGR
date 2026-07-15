use crate::shared::{Entity, Id};

use bon::Builder;
use serde::Serialize;
use sqlx::FromRow;

pub type SourceId = Id<Source>;

#[derive(Debug, Clone, Serialize, FromRow, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Source {
	#[builder(default = SourceId::new())]
	pub id: SourceId,
	pub openalex_id: String,
	pub display_name: String,
	pub ty: String,
	pub issn_l: Option<String>,
	pub issn: Option<Vec<String>>,
}

impl Source {
	pub fn normalize_issn(issn: &str) -> Option<String> {
		let normalized = issn.replace("-", "").to_uppercase();

		if normalized.is_empty() {
			None
		} else {
			Some(normalized)
		}
	}
}

impl Entity for Source {
	fn key_name() -> &'static str {
		"source"
	}
}
