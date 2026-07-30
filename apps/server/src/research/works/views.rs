use crate::{academic::AcademicId, research::*};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkView {
	#[serde(flatten)]
	pub work: Work,
	pub overridden_fields: Vec<String>,
	pub journal_kind: Option<JournalKind>,
	pub research_line_id: Option<Uuid>,
	pub research_line_name: Option<String>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub source: Option<SourceView>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub authorships: Option<Vec<Authorship>>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub topics: Option<Vec<ResearchTopicView>>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub keywords: Option<Vec<ResearchKeywordView>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncResultView {
	pub academic_id: AcademicId,
	pub works_fetched: usize,
	pub works_created: usize,
	pub authorships_inserted: usize,
	pub topics_linked: usize,
	pub keywords_linked: usize,
	pub errors: Vec<String>,
}
