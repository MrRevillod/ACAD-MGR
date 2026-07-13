use crate::research::*;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkDetailView {
	#[serde(flatten)]
	pub work: Work,
	pub source: Option<SourceView>,
	pub authorships: Vec<Authorship>,
	pub topics: Vec<ResearchTopicView>,
	pub keywords: Vec<ResearchKeywordView>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncResultView {
	pub academic_id: uuid::Uuid,
	pub academic_orcid: String,
	pub works_fetched: usize,
	pub works_created: usize,
	pub works_skipped: usize,
	pub authorships_inserted: usize,
	pub topics_linked: usize,
	pub keywords_linked: usize,
	pub errors: Vec<String>,
}
