use super::{JournalKind, SourceId};

use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct SourceView {
	pub id: SourceId,
	pub openalex_id: String,
	pub name: String,
	pub ty: String,
	pub issn: Option<String>,
	pub kind: Option<JournalKind>,
}
