use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Default)]
#[serde(rename_all = "snake_case")]
pub struct GetWorksQuery {
	pub academic_id: Option<Uuid>,
	pub search: Option<String>,
	#[validate(range(min = 1900, max = 2100))]
	pub year_from: Option<i16>,
	#[validate(range(min = 1900, max = 2100))]
	pub year_to: Option<i16>,
	pub r#type: Option<String>,
	pub domain_id: Option<Uuid>,
	pub field_id: Option<Uuid>,
	pub subfield_id: Option<Uuid>,
	pub topic_id: Option<Uuid>,
	pub topic_min_score: Option<f64>,
	pub keyword_id: Option<Uuid>,
	pub keyword_min_score: Option<f64>,
	pub is_accepted: Option<bool>,
	pub is_published: Option<bool>,
	#[validate(range(min = 1, max = 1000))]
	pub size: Option<u32>,
}
