use crate::shared::{Entity, Id};

use bon::Builder;
use serde::Serialize;
use sqlx::FromRow;

pub type ResearchDomainId = Id<ResearchDomain>;

#[derive(Debug, Clone, Serialize, FromRow, Builder)]
pub struct ResearchDomain {
	#[builder(default = ResearchDomainId::new())]
	pub id: ResearchDomainId,
	pub openalex_id: String,
	pub name: String,
}

impl Entity for ResearchDomain {
	fn key_name() -> &'static str {
		"research_domain"
	}
}

pub type ResearchFieldId = Id<ResearchField>;

#[derive(Debug, Clone, Serialize, FromRow, Builder)]
pub struct ResearchField {
	#[builder(default = ResearchFieldId::new())]
	pub id: ResearchFieldId,
	pub openalex_id: String,
	pub name: String,
	pub domain_id: ResearchDomainId,
}

impl Entity for ResearchField {
	fn key_name() -> &'static str {
		"research_field"
	}
}

pub type ResearchSubfieldId = Id<ResearchSubfield>;

#[derive(Debug, Clone, Serialize, FromRow, Builder)]
pub struct ResearchSubfield {
	#[builder(default = ResearchSubfieldId::new())]
	pub id: ResearchSubfieldId,
	pub openalex_id: String,
	pub name: String,
	pub field_id: ResearchFieldId,
}

impl Entity for ResearchSubfield {
	fn key_name() -> &'static str {
		"research_subfield"
	}
}

pub type ResearchTopicId = Id<ResearchTopic>;

#[derive(Debug, Clone, Serialize, FromRow, Builder)]
pub struct ResearchTopic {
	#[builder(default = ResearchTopicId::new())]
	pub id: ResearchTopicId,
	pub openalex_id: String,
	pub name: String,
	pub subfield_id: ResearchSubfieldId,
}

impl Entity for ResearchTopic {
	fn key_name() -> &'static str {
		"research_topic"
	}
}

pub type ResearchKeywordId = Id<ResearchKeyword>;

#[derive(Debug, Clone, Serialize, FromRow, Builder)]
pub struct ResearchKeyword {
	#[builder(default = ResearchKeywordId::new())]
	pub id: ResearchKeywordId,
	pub openalex_id: String,
	pub name: String,
}

impl Entity for ResearchKeyword {
	fn key_name() -> &'static str {
		"research_keyword"
	}
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
