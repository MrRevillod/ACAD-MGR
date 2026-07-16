use super::SourceId;
use super::journal_kind::JournalKind;

use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct SourceView {
	pub id: SourceId,
	pub openalex_id: String,
	pub display_name: String,
	pub ty: String,
	pub issn_l: Option<String>,
	pub issn: Option<Vec<String>>,
	pub kind: Option<JournalKind>,
}
