use crate::academic::AcademicOption;
use crate::research::JournalKind;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, Default)]
#[serde(rename_all = "camelCase")]
pub struct WorksStatsQuery {
	pub journal_kind: Option<JournalKind>,

	pub option: Option<AcademicOption>,
	pub department_id: Option<Uuid>,

	#[validate(range(min = 1900, max = 2100))]
	pub year_from: Option<i16>,

	#[validate(range(min = 1900, max = 2100))]
	pub year_to: Option<i16>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct YearValue {
	pub year: i16,
	pub value: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesStat {
	pub id: Option<String>,
	pub key: String,
	pub values: Vec<YearValue>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorksStatsResponse {
	pub by_journal_kind: Vec<TimeSeriesStat>,
	pub by_option: Vec<TimeSeriesStat>,
	pub by_department: Vec<TimeSeriesStat>,
}

#[derive(Debug, Deserialize, Validate, Default)]
#[serde(rename_all = "camelCase")]
pub struct DepartmentDetailQuery {
	#[validate(range(min = 1900, max = 2100))]
	pub year_from: Option<i16>,

	#[validate(range(min = 1900, max = 2100))]
	pub year_to: Option<i16>,

	pub option: Option<AcademicOption>,

	pub journal_kind: Option<JournalKind>,
}

#[derive(Debug, Deserialize, Validate, Default)]
#[serde(rename_all = "camelCase")]
pub struct AcademicStatsQuery {
	#[validate(range(min = 1900, max = 2100))]
	pub year_from: Option<i16>,

	#[validate(range(min = 1900, max = 2100))]
	pub year_to: Option<i16>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchLineStat {
	pub research_line_id: String,
	pub name: String,
	pub count: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AcademicContribution {
	pub academic_works: i64,
	pub faculty_works: i64,
	pub department_works: i64,
	pub department_name: Option<String>,
	pub dominant_line_works: i64,
	pub line_total_works: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AcademicStatsResponse {
	pub by_research_line: Vec<ResearchLineStat>,
	pub dominant_research_line_id: Option<String>,
	pub by_journal_kind: Vec<TimeSeriesStat>,
	pub contribution: AcademicContribution,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopPublisher {
	pub academic_id: String,
	pub name: String,
	pub total: i64,
	pub scopus: i64,
	pub wos: i64,
	pub unindexed: i64,
	pub option: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DepartmentDetailResponse {
	pub department: String,
	pub total_works: i64,
	pub scopus_count: i64,
	pub wos_count: i64,
	pub teaching_count: i64,
	pub research_count: i64,
	pub top_publishers: Vec<TopPublisher>,
}
