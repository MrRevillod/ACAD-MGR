use sword::prelude::event;

#[event(key = "works.sync-requested")]
pub struct SyncWorksRequest {
	pub user_email: String,
}
