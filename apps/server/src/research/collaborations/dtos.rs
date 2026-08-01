use crate::{academic::AcademicId, research::WorkRef};
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollaborationGraph {
	pub academic_id: AcademicId,
	pub nodes: Vec<CollaborationNode>,
	pub edges: Vec<CollaborationEdge>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollaborationNode {
	pub id: AcademicId,
	pub name: String,
	pub department: String,
	pub total_works: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollaborationEdge {
	pub source_id: AcademicId,
	pub target_id: AcademicId,
	pub weight: i64,
	pub works: Vec<WorkRef>,
}
