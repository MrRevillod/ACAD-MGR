use bon::Builder;

use crate::{
	academic::AcademicCategoryId,
	shared::{Entity, Id},
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Type, Serialize, Deserialize)]
#[sqlx(type_name = "academic_option", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum AcademicOption {
	Teaching,
	Research,
}

pub type AcademicCategoryOptionId = Id<AcademicCategoryOption>;

#[derive(Debug, Clone, Serialize, FromRow, Builder)]
#[serde(rename_all = "camelCase")]
pub struct AcademicCategoryOption {
	#[builder(default = AcademicCategoryOptionId::new())]
	pub id: AcademicCategoryOptionId,
	pub category_id: AcademicCategoryId,
	pub hours: Option<f64>,
	pub option: AcademicOption,
}

impl Entity for AcademicCategoryOption {
	fn key_name() -> &'static str {
		"academic_category_option"
	}
}

#[derive(Debug, Default)]
pub struct AcademicCategoryOptionFilter {
	pub category_id: Option<AcademicCategoryId>,
	pub option: Option<AcademicOption>,
	pub category_name: Option<String>,
}
