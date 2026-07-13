use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Copy, Type, Serialize, Deserialize, Eq, PartialEq)]
#[sqlx(type_name = "journal_kind", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum JournalKind {
	Wos,
	Scopus,
}
