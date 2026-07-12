use crate::shared::{Entity, Id};

use serde::Serialize;
use sqlx::FromRow;

pub type AcademicWorkPositionId = Id<AcademicWorkPosition>;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct AcademicWorkPosition {
	pub id: AcademicWorkPositionId,
	pub name: String,
}

pub struct WorkPositionFilter {
	pub name: Option<String>,
}

impl Entity for AcademicWorkPosition {
	fn key_name() -> &'static str {
		"academic_work_position"
	}
}

impl AcademicWorkPosition {
	pub fn new(name: String) -> Self {
		Self {
			id: AcademicWorkPositionId::new(),
			name,
		}
	}
}
