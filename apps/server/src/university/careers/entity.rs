use crate::{
	shared::model_id,
	university::{Department, DepartmentId},
};

use bon::Builder;
use serde::Serialize;
use toasty::{Deferred, Model};

model_id! {
	struct CareerId,
	key: "career"
}

#[derive(Debug, Clone, Serialize, Builder, Model)]
#[serde(rename_all = "camelCase")]
pub struct Career {
	#[key]
	#[builder(default = CareerId::new())]
	pub id: CareerId,
	pub name: String,

	#[index]
	pub department_id: DepartmentId,

	#[belongs_to]
	#[builder(default)]
	department: Deferred<Department>,
}

#[derive(Debug)]
pub struct CareerFilter {
	pub name: Option<String>,
	pub department_id: Option<DepartmentId>,
}
