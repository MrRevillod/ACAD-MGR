use crate::research::classification::*;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ResearchLineView {
	pub id: Uuid,
	pub name: String,
	pub slug: String,
}

#[derive(Debug, Clone, Default, Deserialize, Validate)]
pub struct WorkClassificationQueryDto {
	pub domain_id: Option<ResearchDomainId>,
	pub field_id: Option<ResearchFieldId>,
	pub subfield_id: Option<ResearchSubfieldId>,
	pub topic_id: Option<ResearchTopicId>,

	#[validate(length(min = 1, max = 255))]
	pub openalex_id: Option<String>,

	#[validate(length(min = 1, max = 255))]
	pub search: Option<String>,
}

#[derive(Debug, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ResearchTopicView {
	pub topic_id: ResearchTopicId,
	pub name: String,
	pub score: f64,
	pub subfield_id: ResearchSubfieldId,
	pub subfield_name: String,
	pub field_id: ResearchFieldId,
	pub field_name: String,
	pub domain_id: ResearchDomainId,
	pub domain_name: String,
}

#[derive(Debug, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ResearchKeywordView {
	pub keyword_id: Uuid,
	pub name: String,
	pub score: f64,
}

impl From<WorkClassificationQueryDto> for ClassificationFilter {
	fn from(dto: WorkClassificationQueryDto) -> Self {
		ClassificationFilter {
			domain_id: dto.domain_id,
			field_id: dto.field_id,
			subfield_id: dto.subfield_id,
			topic_id: dto.topic_id,
			openalex_id: dto.openalex_id,
			search: dto.search,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SubfieldMapping {
	pub subfield_openalex_id: String,
	pub subfield_name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchLineDetail {
	pub id: Uuid,
	pub name: String,
	pub slug: String,
	pub subfields: Vec<SubfieldMapping>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchLinesDetailResponse {
	pub lines: Vec<ResearchLineDetail>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMappingBody {
	pub subfield_openalex_id: String,
	pub research_line_id: Uuid,
}

impl
	From<(
		ResearchTopic,
		ResearchSubfield,
		ResearchField,
		ResearchDomain,
		f64,
	)> for ResearchTopicView
{
	fn from(
		(topic, subfield, field, domain, score): (
			ResearchTopic,
			ResearchSubfield,
			ResearchField,
			ResearchDomain,
			f64,
		),
	) -> Self {
		Self {
			topic_id: topic.id,
			name: topic.name,
			score,
			subfield_id: subfield.id,
			subfield_name: subfield.name,
			field_id: field.id,
			field_name: field.name,
			domain_id: domain.id,
			domain_name: domain.name,
		}
	}
}
