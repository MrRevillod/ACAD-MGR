use crate::research::classification::*;
use serde::Deserialize;
use validator::Validate;

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
