use crate::shared::{Entity, Id};

use bon::Builder;
use serde::Serialize;
use sqlx::FromRow;

pub type DomainId = Id<Domain>;

#[derive(Debug, Clone, Serialize, FromRow, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Domain {
	#[builder(default)]
	pub id: DomainId,
	pub openalex_id: String,
	pub name: String,
}

impl Entity for Domain {
	fn key_name() -> &'static str {
		"domain"
	}
}

pub type FieldId = Id<Field>;

#[derive(Debug, Clone, Serialize, FromRow, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Field {
	#[builder(default)]
	pub id: FieldId,
	pub openalex_id: String,
	pub name: String,
	pub domain_id: DomainId,
}

impl Entity for Field {
	fn key_name() -> &'static str {
		"field"
	}
}

pub type ResearchLineId = Id<ResearchLine>;

#[derive(Debug, Clone, Serialize, FromRow, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ResearchLine {
	#[builder(default)]
	pub id: ResearchLineId,
	pub name: String,
	pub slug: String,
}

impl Entity for ResearchLine {
	fn key_name() -> &'static str {
		"research_line"
	}
}

pub type SubfieldId = Id<Subfield>;

#[derive(Debug, Clone, Serialize, FromRow, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Subfield {
	#[builder(default)]
	pub id: SubfieldId,
	pub openalex_id: String,
	pub name: String,
	pub field_id: FieldId,
	pub research_line_id: Option<ResearchLineId>,
}

impl Entity for Subfield {
	fn key_name() -> &'static str {
		"subfield"
	}
}

pub type TopicId = Id<Topic>;

#[derive(Debug, Clone, Serialize, FromRow, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Topic {
	#[builder(default)]
	pub id: TopicId,
	pub openalex_id: String,
	pub name: String,
	pub subfield_id: SubfieldId,
}

impl Entity for Topic {
	fn key_name() -> &'static str {
		"topic"
	}
}

pub type KeywordId = Id<Keyword>;

#[derive(Debug, Clone, Serialize, FromRow, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Keyword {
	#[builder(default)]
	pub id: KeywordId,
	pub openalex_id: String,
	pub name: String,
}

impl Entity for Keyword {
	fn key_name() -> &'static str {
		"keyword"
	}
}

#[allow(dead_code)]
pub struct ClassificationFilter {
	pub domain_id: Option<DomainId>,
	pub field_id: Option<FieldId>,
	pub subfield_id: Option<SubfieldId>,
	pub topic_id: Option<TopicId>,
	pub openalex_id: Option<String>,
	pub search: Option<String>,
	pub limit: Option<i64>,
}
