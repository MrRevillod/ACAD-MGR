use crate::research::sources::SourceId;
use crate::shared::{Entity, Id};

use bon::Builder;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use std::str::FromStr;

pub type WorkId = Id<Work>;

#[derive(Debug, Clone, Copy, Type, Serialize, Deserialize, Eq, PartialEq)]
#[sqlx(type_name = "work_type", rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
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

#[derive(Debug, Clone, Serialize, FromRow, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Work {
	#[builder(default = WorkId::new())]
	pub id: WorkId,
	pub openalex_id: String,
	pub title: String,
	pub r#abstract: Option<String>,
	pub doi: Option<String>,
	pub publication_date: Option<NaiveDate>,
	pub publication_year: Option<i16>,
	pub ty: WorkType,
	pub lang: String,
	pub is_accepted: bool,
	pub is_published: bool,
	pub primary_source_id: Option<SourceId>,
	pub journal_kind: Option<String>,
}

impl Entity for Work {
	fn key_name() -> &'static str {
		"work"
	}
}
