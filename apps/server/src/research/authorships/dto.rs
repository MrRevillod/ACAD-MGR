use crate::research::authorships::entity::AuthorshipPosition;
use crate::research::works::WorkId;

pub struct NewAuthorship {
	pub work_id: WorkId,
	pub orcid: String,
	pub name: String,
	pub is_external: bool,
	pub is_corresponding: bool,
	pub affiliations: Vec<String>,
	pub position: AuthorshipPosition,
}
