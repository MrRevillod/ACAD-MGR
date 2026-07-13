use crate::shared::{Entity, Id};

use bon::Builder;
use serde::Serialize;
use sqlx::FromRow;

pub type SourceId = Id<Source>;

#[derive(Debug, Clone, Serialize, FromRow, Builder)]
pub struct Source {
	#[builder(default = SourceId::new())]
	pub id: SourceId,
	pub openalex_id: String,
	pub display_name: String,
	pub ty: String,
}

impl Entity for Source {
	fn key_name() -> &'static str {
		"source"
	}
}
