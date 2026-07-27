use crate::academic::*;
use crate::shared::model_id;
use crate::university::*;

use bon::Builder;
use jiff::{Timestamp, civil::Date};
use serde::{Deserialize, Serialize};
use toasty::{Deferred, Embed, Model};

model_id! {
	struct AcademicId,
	key: "academic_id"
}

#[derive(Debug, Clone, Copy, Embed, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
#[column(rename_all = "UPPERCASE")]
pub enum Sex {
	H,
	M,
	O,
}

#[derive(Debug, Clone, Serialize, Model, Builder)]
pub struct Academic {
	#[key]
	#[builder(default = AcademicId::new())]
	pub id: AcademicId,

	#[unique]
	pub rut: String,

	#[unique]
	pub email: String,

	#[unique]
	pub orcid: Option<String>,

	pub names: String,
	pub paternal_surname: String,
	pub maternal_surname: String,
	pub sex: Sex,
	pub joined_at: Date,
	pub birth_date: Date,
	pub jce: f64,
	pub city: String,
	pub annual_discount_hours: f64,

	#[builder(default = Timestamp::now())]
	pub updated_at: Timestamp,

	#[index]
	pub nationality_code: String,

	#[index]
	pub department_id: DepartmentId,

	#[index]
	pub career_id: Option<CareerId>,

	#[index]
	pub category_option_id: AcademicCategoryOptionId,

	#[index]
	pub work_position_id: AcademicWorkPositionId,

	#[has_many]
	#[builder(default)]
	pub degrees: Deferred<Vec<Degree>>,

	#[belongs_to(key = nationality_code, references = code)]
	#[builder(default)]
	pub nationality: Deferred<Country>,

	#[belongs_to]
	#[builder(default)]
	pub department: Deferred<Department>,

	#[belongs_to]
	#[builder(default)]
	pub career: Deferred<Option<Career>>,

	#[belongs_to]
	#[builder(default)]
	pub category_option: Deferred<AcademicCategoryOption>,

	#[belongs_to]
	#[builder(default)]
	pub work_position: Deferred<AcademicWorkPosition>,
}

#[derive(Debug)]
pub struct AcademicListFilter {
	pub search: Option<String>,
	pub career_id: Option<CareerId>,
	pub department_id: Option<DepartmentId>,
	pub category_id: Option<AcademicCategoryId>,
	pub planta: Option<AcademicPlanta>,
	pub option: Option<AcademicOption>,
}

impl Academic {
	pub fn full_name(&self) -> String {
		format!(
			"{} {} {}",
			self.names, self.paternal_surname, self.maternal_surname
		)
	}
}
