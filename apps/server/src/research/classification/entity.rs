use crate::shared::model_id;
use bon::Builder;
use serde::Serialize;
use toasty::{Deferred, Model};

#[derive(Debug, Clone, Serialize, Model, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ResearchDomain {
	#[key]
	#[builder(default = ResearchDomainId::new())]
	pub id: ResearchDomainId,
	pub name: String,

	#[unique]
	pub openalex_id: String,

	#[has_many]
	pub fields: Deferred<Vec<ResearchField>>,
}

#[derive(Debug, Clone, Serialize, Model, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ResearchField {
	#[key]
	#[builder(default = ResearchFieldId::new())]
	pub id: ResearchFieldId,
	pub name: String,

	#[unique]
	pub openalex_id: String,

	#[index]
	pub domain_id: ResearchDomainId,

	#[belongs_to]
	pub domain: Deferred<ResearchDomain>,

	#[has_many]
	pub subfields: Deferred<Vec<ResearchSubfield>>,
}

#[derive(Debug, Clone, Serialize, Model, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ResearchSubfield {
	#[key]
	#[builder(default = ResearchSubfieldId::new())]
	pub id: ResearchSubfieldId,
	pub name: String,

	#[unique]
	pub openalex_id: String,

	#[index]
	pub field_id: ResearchFieldId,

	#[index]
	pub research_line_id: Option<ResearchLineId>,

	#[belongs_to]
	pub field: Deferred<ResearchField>,

	#[belongs_to]
	pub research_line: Option<Deferred<ResearchLine>>,

	#[has_many]
	pub topics: Deferred<Vec<ResearchTopic>>,
}

#[derive(Debug, Clone, Serialize, Model, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ResearchTopic {
	#[key]
	#[builder(default = ResearchTopicId::new())]
	pub id: ResearchTopicId,

	#[unique]
	pub openalex_id: String,
	pub name: String,

	#[index]
	pub subfield_id: ResearchSubfieldId,

	#[belongs_to]
	pub subfield: Deferred<ResearchSubfield>,
}

#[derive(Debug, Clone, Serialize, Model, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ResearchKeyword {
	#[key]
	#[builder(default = ResearchKeywordId::new())]
	pub id: ResearchKeywordId,

	#[unique]
	pub openalex_id: String,
	pub name: String,
}

#[derive(Debug, Clone, Serialize, Model, Builder)]
pub struct ResearchLine {
	#[key]
	pub id: ResearchLineId,
	pub name: String,
	pub slug: String,

	#[has_many]
	pub subfields: Deferred<Vec<ResearchSubfield>>,
}

#[allow(dead_code)]
pub struct ClassificationFilter {
	pub domain_id: Option<ResearchDomainId>,
	pub field_id: Option<ResearchFieldId>,
	pub subfield_id: Option<ResearchSubfieldId>,
	pub topic_id: Option<ResearchTopicId>,
	pub openalex_id: Option<String>,
	pub search: Option<String>,
}

model_id! {
	struct ResearchLineId,
	key: "research_line"
}

model_id! {
	struct ResearchDomainId,
	key: "research_domain"
}

model_id! {
	struct ResearchFieldId,
	key: "research_field"
}

model_id! {
	struct ResearchSubfieldId,
	key: "research_subfield"
}

model_id! {
	struct ResearchTopicId,
	key: "research_topic"
}
model_id! {
	struct ResearchKeywordId,
	key: "research_keyword"
}
