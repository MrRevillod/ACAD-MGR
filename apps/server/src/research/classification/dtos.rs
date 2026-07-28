use crate::research::classification::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Default, Deserialize, Validate)]
pub struct WorkClassificationQueryDto {
	pub domain_id: Option<DomainId>,
	pub field_id: Option<FieldId>,
	pub subfield_id: Option<SubfieldId>,
	pub topic_id: Option<TopicId>,

	#[validate(length(min = 1, max = 255))]
	pub openalex_id: Option<String>,

	#[validate(length(min = 1, max = 255))]
	pub search: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopicView {
	pub topic_id: TopicId,
	pub name: String,
	pub score: f64,
	pub subfield_id: SubfieldId,
	pub subfield_name: String,
	pub field_id: FieldId,
	pub field_name: String,
	pub domain_id: DomainId,
	pub domain_name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeywordView {
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMappingBody {
	pub subfield_id: SubfieldId,
	pub research_line_id: ResearchLineId,
}

impl From<(Topic, Subfield, Field, Domain, f64)> for TopicView {
	fn from(
		(topic, subfield, field, domain, score): (Topic, Subfield, Field, Domain, f64),
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
