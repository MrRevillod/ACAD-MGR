use crate::research::SourceId;
use crate::shared::{Entity, Id};

use bon::Builder;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Type)]
#[sqlx(type_name = "work_overrides")]
#[serde(rename_all = "camelCase", default)]
pub struct WorkOverrides {
	pub title: Option<String>,
	pub abstract_text: Option<String>,
	pub doi: Option<String>,
	pub publication_year: Option<i16>,
	pub is_accepted: Option<bool>,
	pub is_published: Option<bool>,
	pub research_line_id: Option<Uuid>,
}

impl WorkOverrides {
	pub fn overridden_field_names(&self) -> Vec<String> {
		let mut fields = Vec::new();

		if self.title.is_some() {
			fields.push("title".into());
		}
		if self.abstract_text.is_some() {
			fields.push("abstractText".into());
		}
		if self.doi.is_some() {
			fields.push("doi".into());
		}
		if self.publication_year.is_some() {
			fields.push("publicationYear".into());
		}
		if self.is_accepted.is_some() {
			fields.push("isAccepted".into());
		}
		if self.is_published.is_some() {
			fields.push("isPublished".into());
		}
		if self.research_line_id.is_some() {
			fields.push("researchLineId".into());
		}

		fields
	}
}

pub type WorkId = Id<Work>;

#[derive(Debug, Clone, Builder)]
pub struct WorkTopicScore {
	pub work_id: WorkId,
	pub topic_id: Uuid,
	pub score: f64,
}

#[derive(Debug, Clone, Builder)]
pub struct WorkKeywordScore {
	pub work_id: WorkId,
	pub keyword_id: Uuid,
	pub score: f64,
}

#[derive(Debug, Clone, Serialize, FromRow, Builder, Default)]
#[serde(rename_all = "camelCase")]
pub struct Work {
	#[builder(default = WorkId::new())]
	pub id: WorkId,
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
	pub source_id: Option<SourceId>,

	#[builder(default)]
	pub updated_at: DateTime<Utc>,

	#[serde(skip)]
	#[builder(default)]
	pub overrides: WorkOverrides,
}

impl Work {
	pub fn resolve(&self) -> Self {
		let o = &self.overrides;

		Self {
			title: o.title.clone().unwrap_or_else(|| self.title.clone()),
			abstract_text: o.abstract_text.clone().or_else(|| self.abstract_text.clone()),
			doi: o.doi.clone().or_else(|| self.doi.clone()),
			publication_year: o.publication_year.or(self.publication_year),
			is_accepted: o.is_accepted.unwrap_or(self.is_accepted),
			is_published: o.is_published.unwrap_or(self.is_published),
			..self.clone()
		}
	}
}

impl Entity for Work {
	fn key_name() -> &'static str {
		"work"
	}
}

#[derive(Debug, Clone, Copy, Type, Serialize, Deserialize, Eq, PartialEq, Hash, Default)]
#[sqlx(type_name = "work_type", rename_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum WorkType {
	#[default]
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
