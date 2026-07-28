use crate::{shared::model_id, university::Department};
use bon::Builder;
use serde::Serialize;
use toasty::{Deferred, Model};

model_id! {
	struct FacultyId, key: "faculty"
}

#[derive(Debug, Clone, Serialize, Builder, Model)]
pub struct Faculty {
	#[key]
	#[builder(default = FacultyId::new())]
	pub id: FacultyId,
	pub name: String,

	#[has_many]
	#[builder(default)]
	pub departments: Deferred<Vec<Department>>,
}

pub struct FacultyFilter {
	pub name: Option<String>,
}
