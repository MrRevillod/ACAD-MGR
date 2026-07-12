use crate::academic::{AcademicId, AcademicOption, AcademicPlanta, Sex};

use chrono::NaiveDate;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct AcademicView {
	pub id: AcademicId,
	pub names: String,
	pub paternal_surname: String,
	pub maternal_surname: String,
	pub email: String,
	pub orcid: Option<String>,
	pub sex: Sex,
	pub birth_date: NaiveDate,
	pub joined_at: NaiveDate,
	pub work_position: Option<String>,
	pub department: String,
	pub career: Option<String>,
	pub jce: f64,
	pub category: String,
	pub planta: AcademicPlanta,
	pub option: AcademicOption,
	pub acad_category_hours: Option<f64>,
	pub annual_discount_hours: f64,
	pub nationality: String,
	pub city: String,
}
