use crate::{
	academic::AcademicId,
	research::{Source, SourceId},
	shared::model_id,
};

use bon::Builder;
use jiff::civil::Date;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use toasty::{Deferred, Embed, Model};

#[derive(Debug, Clone, Copy, Embed, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
#[column(rename_all = "lowercase")]
pub enum AuthorshipPosition {
	First,
	Middle,
	Last,
}

#[derive(Debug, Clone, Serialize, Model)]
#[key(work_id, orcid)]
#[serde(rename_all = "camelCase")]
pub struct Authorship {
	pub work_id: WorkId,
	pub orcid: String,
	pub name: String,
	pub is_external: bool,
	pub is_corresponding: bool,
	pub affiliations: Vec<String>,
	pub position: AuthorshipPosition,

	#[index]
	pub academic_id: Option<AcademicId>,

	#[belongs_to]
	pub work: Deferred<Work>,
}

#[derive(Debug, Clone, Serialize, Model, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Work {
	#[key]
	#[builder(default = WorkId::new())]
	pub id: WorkId,

	#[unique]
	pub openalex_id: String,
	pub title: String,
	pub r#abstract: Option<String>,
	pub doi: Option<String>,
	pub publication_date: Option<Date>,
	pub publication_year: Option<i16>,
	pub ty: WorkType,
	pub lang: String,
	pub is_accepted: bool,
	pub is_published: bool,

	#[index]
	pub source_id: Option<SourceId>,

	#[column(type = "jsonb")]
	pub overrides: serde_json::Value,

	#[belongs_to]
	pub source: Deferred<Option<Source>>,

	#[has_many]
	pub authorships: Deferred<Vec<Authorship>>,
}

model_id! {
	struct WorkId, key: "work"
}

#[derive(Debug, Clone, Copy, Embed, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[column(rename_all = "kebab-case")]
pub enum WorkType {
	Article,
	Book,
	BookChapter,
	BookReview,
	ConferenceAbstract,
	ConferencePaper,
	DataPaper,
	Dissertation,
	Editorial,
	Erratum,
	Letter,
	Libguide,
	Other,
	Paratext,
	PeerReview,
	Preprint,
	ReferenceEntry,
	Report,
	Retraction,
	Review,
	Software,
	SoftwarePaper,
	Standard,
	SupplementaryMaterials,
}

impl FromStr for WorkType {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"article" => Ok(WorkType::Article),
			"book" => Ok(WorkType::Book),
			"book-chapter" => Ok(WorkType::BookChapter),
			"book-review" => Ok(WorkType::BookReview),
			"conference-abstract" => Ok(WorkType::ConferenceAbstract),
			"conference-paper" => Ok(WorkType::ConferencePaper),
			"data-paper" => Ok(WorkType::DataPaper),
			"dissertation" => Ok(WorkType::Dissertation),
			"editorial" => Ok(WorkType::Editorial),
			"erratum" => Ok(WorkType::Erratum),
			"letter" => Ok(WorkType::Letter),
			"libguide" => Ok(WorkType::Libguide),
			"other" => Ok(WorkType::Other),
			"paratext" => Ok(WorkType::Paratext),
			"peer-review" => Ok(WorkType::PeerReview),
			"preprint" => Ok(WorkType::Preprint),
			"reference-entry" => Ok(WorkType::ReferenceEntry),
			"report" => Ok(WorkType::Report),
			"retraction" => Ok(WorkType::Retraction),
			"review" => Ok(WorkType::Review),
			"software" => Ok(WorkType::Software),
			"software-paper" => Ok(WorkType::SoftwarePaper),
			"standard" => Ok(WorkType::Standard),
			"supplementary-materials" => Ok(WorkType::SupplementaryMaterials),
			other => Err(format!("unknown work type: {other}")),
		}
	}
}
