use crate::research::*;
use o2o::o2o as FromImpl;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkView {
	#[serde(flatten)]
	pub work: Work,
	pub research_line: Option<ResearchLine>,
	pub topics: Vec<WorkTopicView>,
	pub keywords: Vec<WorkKeywordView>,
}

#[derive(Debug, Serialize, FromImpl)]
#[from_owned(WorkTopicScore)]
#[serde(rename_all = "camelCase")]
pub struct WorkTopicView {
	pub topic_id: TopicId,

	#[from(@.topic.name)]
	pub name: String,
	pub score: f64,

	#[from(@.topic.subfield_id)]
	pub subfield_id: SubfieldId,

	#[from(@.topic.subfield.name)]
	pub subfield_name: String,

	#[from(@.topic.subfield.field_id)]
	pub field_id: FieldId,

	#[from(@.topic.subfield.field.name)]
	pub field_name: String,

	#[from(@.topic.subfield.field.domain_id)]
	pub domain_id: DomainId,

	#[from(@.topic.subfield.field.domain.name)]
	pub domain_name: String,
}

#[derive(Debug, Serialize, FromImpl)]
#[from_owned(WorkKeyWordScore)]
#[serde(rename_all = "camelCase")]
pub struct WorkKeywordView {
	#[from(@.keyword_id)]
	pub keyword_id: KeywordId,

	#[from(@.keyword.name)]
	pub name: String,
	pub score: f64,
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
