use crate::{
	shared::model_id,
	university::{Career, Faculty, FacultyId},
};

use bon::Builder;
use serde::Serialize;
use toasty::{Deferred, Model};

model_id! {
	struct DepartmentId, key: "department"
}

#[derive(Debug, Clone, Serialize, Model, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Department {
	#[key]
	#[builder(default = DepartmentId::new())]
	pub id: DepartmentId,
	pub name: String,

	#[index]
	pub faculty_id: FacultyId,

	#[belongs_to]
	#[builder(default)]
	pub faculty: Deferred<Faculty>,

	#[has_many]
	#[builder(default)]
	pub careers: Deferred<Vec<Career>>,
}

#[derive(Debug)]
pub struct DepartmentFilter {
	pub name: Option<String>,
	pub faculty_id: Option<FacultyId>,
}
