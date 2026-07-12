use crate::research::works::entity::WorkType;

use crate::research::works::entity::{AuthorshipPosition, SourceId, WorkId};
use chrono::NaiveDate;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkView {
	pub id: WorkId,
	pub openalex_id: String,
	pub title: String,
	pub r#abstract: Option<String>,
	pub doi: Option<String>,
	pub publication_date: Option<NaiveDate>,
	pub publication_year: Option<i16>,
	pub r#type: WorkType,
	pub lang: String,
	pub is_accepted: bool,
	pub is_published: bool,
	pub primary_source_id: Option<SourceId>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceView {
	pub id: SourceId,
	pub openalex_id: String,
	pub display_name: String,
	pub ty: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorshipView {
	pub orcid: String,
	pub name: String,
	pub is_external: bool,
	pub is_corresponding: bool,
	pub affiliations: Vec<String>,
	pub position: AuthorshipPosition,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkTopicView {
	pub topic_id: Uuid,
	pub name: String,
	pub score: f64,
	pub subfield_id: Uuid,
	pub subfield_name: String,
	pub field_id: Uuid,
	pub field_name: String,
	pub domain_id: Uuid,
	pub domain_name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkKeywordView {
	pub keyword_id: Uuid,
	pub name: String,
	pub score: f64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkDetailView {
	#[serde(flatten)]
	pub work: WorkView,
	pub source: Option<SourceView>,
	pub authorships: Vec<AuthorshipView>,
	pub topics: Vec<WorkTopicView>,
	pub keywords: Vec<WorkKeywordView>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncResultView {
	pub academic_id: Uuid,
	pub academic_orcid: String,
	pub works_fetched: usize,
	pub works_created: usize,
	pub works_skipped: usize,
	pub authorships_inserted: usize,
	pub topics_linked: usize,
	pub keywords_linked: usize,
	pub errors: Vec<String>,
}
