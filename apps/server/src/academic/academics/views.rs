use crate::academic::{Academic, AcademicId, AcademicOption, AcademicPlanta, Sex};

use jiff::civil::Date;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AcademicView {
	pub id: AcademicId,
	pub names: String,
	pub paternal_surname: String,
	pub maternal_surname: String,
	pub email: String,
	pub orcid: Option<String>,
	pub sex: Sex,
	pub birth_date: Date,
	pub joined_at: Date,
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

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AcademicPublicView {
	pub id: AcademicId,
	pub names: String,
	pub paternal_surname: String,
	pub maternal_surname: String,
	pub email: String,
	pub orcid: Option<String>,
	pub sex: Sex,
	pub birth_date: Date,
	pub joined_at: Date,
	pub department: String,
	pub career: Option<String>,
	pub nationality: String,
	pub city: String,
}

impl From<AcademicView> for AcademicPublicView {
	fn from(view: AcademicView) -> Self {
		Self {
			id: view.id,
			names: view.names,
			paternal_surname: view.paternal_surname,
			maternal_surname: view.maternal_surname,
			email: view.email,
			orcid: view.orcid,
			sex: view.sex,
			birth_date: view.birth_date,
			joined_at: view.joined_at,
			department: view.department,
			career: view.career,
			nationality: view.nationality,
			city: view.city,
		}
	}
}

impl From<Academic> for AcademicView {
	fn from(a: Academic) -> Self {
		AcademicView {
			id: a.id,
			names: a.names,
			paternal_surname: a.paternal_surname,
			maternal_surname: a.maternal_surname,
			email: a.email,
			orcid: a.orcid,
			sex: a.sex,
			birth_date: a.birth_date,
			joined_at: a.joined_at,
			work_position: Some(a.work_position.get().name),
			department: a.department.get().name,
			career: a.career.get().name,
			jce: a.jce,
			category: a.category_option.get().category.get().name,
			planta: a.category_option.get().category.get().planta,
			option: a.category_option.get().option,
			acad_category_hours: a.category_option.get().hours,
			annual_discount_hours: a.annual_discount_hours,
			nationality: a.nationality_code,
			city: a.city,
		}
	}
}
