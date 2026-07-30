use crate::research::JournalKind;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetWorksQuery {
	pub academic_id: Option<Uuid>,
	pub search: Option<String>,

	#[validate(range(min = 1900, max = 2100))]
	pub year_from: Option<i16>,

	#[validate(range(min = 1900, max = 2100))]
	pub year_to: Option<i16>,

	pub is_accepted: Option<bool>,
	pub is_published: Option<bool>,
	pub department_id: Option<Uuid>,
	pub career_id: Option<Uuid>,
	pub journal_kind: Option<JournalKind>,
	pub research_line_id: Option<Uuid>,

	#[validate(range(min = 1, max = 1000))]
	pub size: Option<u32>,
}

pub struct WorkImportProcessStats {
	pub was_inserted: bool,
	pub authorships: usize,
	pub topics: usize,
	pub keywords: usize,
}

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkOverridesInput {
	pub title: Option<Option<String>>,
	pub abstract_text: Option<Option<String>>,
	pub doi: Option<Option<String>>,
	pub publication_year: Option<Option<i16>>,
	pub is_accepted: Option<Option<bool>>,
	pub is_published: Option<Option<bool>>,
	pub research_line_id: Option<Option<Uuid>>,
}
