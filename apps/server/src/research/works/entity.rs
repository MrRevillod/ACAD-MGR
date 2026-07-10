use crate::shared::{Entity, Id};

use bon::Builder;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

pub type WorkId = Id<Work>;
pub type SourceId = Id<Source>;

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

#[derive(Debug, Clone, Copy, Type, Serialize, Deserialize, Eq, PartialEq)]
#[sqlx(type_name = "authorship_position", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum AuthorshipPosition {
    First,
    Middle,
    Last,
}

#[derive(Debug, Clone, Serialize, FromRow, Builder)]
pub struct Source {
    #[builder(default = SourceId::new())]
    pub id: SourceId,
    pub openalex_id: String,
    pub display_name: String,
    pub ty: String,
}

impl Entity for Source {
    fn key_name() -> &'static str {
        "source"
    }
}

#[derive(Debug, Clone, Serialize, FromRow, Builder)]
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
}

impl Entity for Work {
    fn key_name() -> &'static str {
        "work"
    }
}
