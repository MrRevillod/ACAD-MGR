use crate::shared::model_id;
use bon::Builder;
use serde::{Deserialize, Serialize};
use toasty::{Embed, Model};

model_id! {
	struct AcademicCategoryId,
	key: "academic_category"
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Embed)]
#[serde(rename_all = "lowercase")]
#[column(rename_all = "lowercase")]
pub enum AcademicPlanta {
	Adjunta,
	Permanente,
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder, Model)]
pub struct AcademicCategory {
	#[key]
	#[builder(default = AcademicCategoryId::new())]
	pub id: AcademicCategoryId,

	pub name: String,
	pub planta: AcademicPlanta,
}

#[derive(Debug)]
pub struct AcademicCategoryFilter {
	pub name: Option<String>,
	pub planta: Option<AcademicPlanta>,
}
