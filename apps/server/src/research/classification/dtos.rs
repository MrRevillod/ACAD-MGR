use crate::research::classification::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Clone, Default, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct WorkClassificationQueryDto {
	pub domain_id: Option<DomainId>,
	pub field_id: Option<FieldId>,
	pub subfield_id: Option<SubfieldId>,
	pub topic_id: Option<TopicId>,

	#[validate(length(min = 1, max = 255))]
	pub openalex_id: Option<String>,

	#[validate(length(min = 1, max = 255))]
	pub search: Option<String>,
	pub limit: Option<i64>,
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
			limit: dto.limit,
		}
	}
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMappingBody {
	pub subfield_openalex_id: String,
	pub research_line_id: ResearchLineId,
}
