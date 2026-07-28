use crate::{
	research::{Work, WorkKeyWordScore, WorkTopicScore},
	shared::model_id,
};
use bon::Builder;
use serde::Serialize;
use toasty::{Deferred, Model};

#[derive(Debug, Clone, Serialize, Model, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Domain {
	#[key]
	#[builder(default)]
	pub id: DomainId,
	pub name: String,

	#[unique]
	pub openalex_id: String,
}

#[derive(Debug, Clone, Serialize, Model, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Field {
	#[key]
	#[builder(default)]
	pub id: FieldId,
	pub name: String,

	#[unique]
	pub openalex_id: String,

	#[index]
	pub domain_id: DomainId,

	#[belongs_to]
	pub domain: Domain,
}

#[derive(Debug, Clone, Serialize, Model, Builder)]
pub struct ResearchLine {
	#[key]
	#[builder(default)]
	pub id: ResearchLineId,
	pub name: String,
	pub slug: String,

	#[has_many]
	#[serde(skip)]
	pub subfields: Deferred<Vec<Subfield>>,
}

#[derive(Debug, Clone, Serialize, Model, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Subfield {
	#[key]
	#[builder(default)]
	pub id: SubfieldId,
	pub name: String,

	#[unique]
	pub openalex_id: String,

	#[index]
	pub field_id: FieldId,

	#[index]
	pub research_line_id: Option<ResearchLineId>,

	#[belongs_to]
	pub field: Field,

	#[belongs_to]
	pub research_line: Option<ResearchLine>,
}

#[derive(Debug, Clone, Serialize, Model, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Topic {
	#[key]
	#[builder(default)]
	pub id: TopicId,

	#[unique]
	pub openalex_id: String,
	pub name: String,

	#[index]
	pub subfield_id: SubfieldId,

	#[belongs_to]
	pub subfield: Subfield,

	#[has_many]
	pub work_scores: Deferred<Vec<WorkTopicScore>>,

	#[has_many(via = work_scores.work)]
	pub works: Deferred<Vec<Work>>,
}

#[derive(Debug, Clone, Serialize, Model, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Keyword {
	#[key]
	#[builder(default)]
	pub id: KeywordId,

	#[unique]
	pub openalex_id: String,
	pub name: String,

	#[has_many]
	pub work_scores: Deferred<Vec<WorkKeyWordScore>>,

	#[has_many(via = work_scores.work)]
	pub works: Deferred<Vec<Work>>,
}

#[allow(dead_code)]
pub struct ClassificationFilter {
	pub domain_id: Option<DomainId>,
	pub field_id: Option<FieldId>,
	pub subfield_id: Option<SubfieldId>,
	pub topic_id: Option<TopicId>,
	pub openalex_id: Option<String>,
	pub search: Option<String>,
}

model_id! {
	struct ResearchLineId, key: "research_line"
}

model_id! {
	struct DomainId, key: "domain"
}

model_id! {
	struct FieldId, key: "field"
}

model_id! {
	struct SubfieldId, key: "subfield"
}

model_id! {
	struct TopicId, key: "topic"
}
model_id! {
	struct KeywordId, key: "keyword"
}
