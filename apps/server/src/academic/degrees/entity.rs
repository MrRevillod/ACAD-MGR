use crate::{
	academic::{Academic, AcademicId},
	shared::model_id,
	university::Country,
};

use bon::Builder;
use jiff::civil::Date;
use serde::{Deserialize, Serialize};
use toasty::{Deferred, Embed, Model};

#[derive(Debug, Clone, Embed, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[column(rename_all = "lowercase")]
pub enum DegreeKind {
	Base,
	Advanced,
}

model_id! {
	struct DegreeId,
	key: "degree"
}

#[derive(Debug, Clone, Serialize, Model, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Degree {
	#[key]
	#[builder(default = DegreeId::new())]
	pub id: DegreeId,
	pub name: String,
	pub university: String,
	pub obtained_at: Date,
	pub kind: DegreeKind,

	#[index]
	pub academic_id: AcademicId,

	#[index]
	pub country_code: String,

	#[belongs_to]
	#[builder(default)]
	pub academic: Deferred<Academic>,

	#[belongs_to(key = country_code, references = code)]
	#[builder(default)]
	pub country: Deferred<Country>,
}
