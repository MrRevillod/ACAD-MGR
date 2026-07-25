use crate::model_id;
use bon::Builder;
use serde::Serialize;
use toasty::Model;

model_id! {
	struct AcademicWorkPositionId,
	key: "academic_work_position"
}

#[derive(Debug, Clone, Serialize, Builder, Model)]
pub struct AcademicWorkPosition {
	#[key]
	#[builder(default = AcademicWorkPositionId::new())]
	pub id: AcademicWorkPositionId,
	pub name: String,
}
