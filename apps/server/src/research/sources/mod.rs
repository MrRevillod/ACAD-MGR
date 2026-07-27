mod repository;

pub use repository::SourcesRepository;

use crate::{research::Work, shared::model_id};
use bon::Builder;
use serde::{Deserialize, Serialize};
use toasty::{Deferred, Embed, Model};

model_id! {
	struct SourceId,
	key: "source"
}

#[derive(Debug, Clone, Serialize, Model, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Source {
	#[key]
	#[builder(default = SourceId::new())]
	pub id: SourceId,

	#[unique]
	pub openalex_id: String,
	pub display_name: String,
	pub ty: String,
	pub issn: Vec<String>,

	#[has_one]
	pub journal_issn: Deferred<Option<JournalIssn>>,

	#[has_many]
	pub works: Deferred<Vec<Work>>,
}

#[derive(Debug, Clone, Serialize, Model)]
pub struct JournalIssn {
	#[key]
	#[auto]
	pub id: i64,

	#[unique]
	pub issn: String,
	pub kind: JournalKind,

	#[index]
	pub source_id: Option<SourceId>,

	#[belongs_to]
	pub source: Deferred<Option<Source>>,
}

#[derive(Debug, Clone, Copy, Embed, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
#[column(rename_all = "lowercase")]
pub enum JournalKind {
	Wos,
	Scopus,
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
