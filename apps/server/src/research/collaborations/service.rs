use crate::{
	academic::AcademicId,
	research::{
		CollaborationEdge, CollaborationGraph, CollaborationNode, CollaborationsRepository, WorkId,
		WorkRef,
	},
	shared::AppResult,
};
use std::collections::HashMap;
use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct CollaborationsService {
	repository: Arc<CollaborationsRepository>,
}

impl CollaborationsService {
	pub async fn get_collaborations(
		&self,
		academic_id: AcademicId,
	) -> AppResult<CollaborationGraph> {
		let (nodes, edges) = tokio::join!(
			self.repository.find_nodes(&academic_id),
			self.repository.find_edges(&academic_id),
		);

		let nodes = nodes?;
		let edges = edges?;

		let work_ids = edges
			.iter()
			.flat_map(|e| e.work_ids.iter().copied())
			.collect::<Vec<WorkId>>();

		let works = self.repository.find_works(&work_ids).await?;
		let works_by_id: HashMap<WorkId, WorkRef> = works.into_iter().map(|w| (w.id, w)).collect();

		let nodes = nodes
			.into_iter()
			.map(|n| CollaborationNode {
				id: n.id,
				name: format!("{} {} {}", n.names, n.paternal_surname, n.maternal_surname),
				department: n.department,
				total_works: n.total_works,
			})
			.collect();

		let edges = edges
			.into_iter()
			.map(|e| CollaborationEdge {
				source_id: e.source_id,
				target_id: e.target_id,
				weight: e.weight,
				works: e
					.work_ids
					.iter()
					.filter_map(|id| works_by_id.get(id).cloned())
					.collect(),
			})
			.collect();

		Ok(CollaborationGraph {
			academic_id,
			nodes,
			edges,
		})
	}
}
