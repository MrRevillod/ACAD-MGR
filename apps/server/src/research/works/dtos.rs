use crate::research::{JournalKind, SourceId, WorkType};
use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

pub struct NewWork {
	pub openalex_id: String,
	pub title: String,
	pub abstract_text: Option<String>,
	pub doi: Option<String>,
	pub publication_date: Option<NaiveDate>,
	pub publication_year: Option<i16>,
	pub ty: WorkType,
	pub lang: String,
	pub is_accepted: bool,
	pub is_published: bool,
	pub primary_source_id: Option<SourceId>,
}

#[derive(Debug, Deserialize, Validate, Default)]
#[serde(rename_all = "snake_case")]
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

	#[validate(range(min = 1, max = 1000))]
	pub size: Option<u32>,
}

pub struct WorkImportProcessStats {
	pub was_inserted: bool,
	pub authorships: usize,
	pub topics: usize,
	pub keywords: usize,
}
