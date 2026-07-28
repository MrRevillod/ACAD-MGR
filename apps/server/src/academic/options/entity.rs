use crate::{
	academic::{AcademicCategory, AcademicCategoryId},
	shared::model_id,
};

use bon::Builder;
use serde::{Deserialize, Serialize};
use toasty::{Deferred, Embed, Model};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Embed, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[column(rename_all = "lowercase")]
pub enum AcademicOption {
	Teaching,
	Research,
}

model_id! {
	struct AcademicCategoryOptionId,
	key: "academic_category_option"
}

#[derive(Debug, Clone, Serialize, Model, Builder)]
#[serde(rename_all = "camelCase")]
pub struct AcademicCategoryOption {
	#[key]
	#[builder(default = AcademicCategoryOptionId::new())]
	pub id: AcademicCategoryOptionId,

	#[index]
	pub category_id: AcademicCategoryId,

	pub hours: Option<f64>,
	pub option: AcademicOption,

	#[belongs_to]
	#[builder(default)]
	pub category: Deferred<AcademicCategory>,
}

#[derive(Debug, Default)]
pub struct AcademicCategoryOptionFilter {
	pub option: Option<AcademicOption>,
	pub category_id: Option<AcademicCategoryId>,
	pub category_name: Option<String>,
}
