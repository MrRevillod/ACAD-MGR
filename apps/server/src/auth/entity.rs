use crate::{
	auth::{User, UserId},
	model_id,
};

use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use toasty::{Deferred, Model};

model_id! {
	struct SessionId, key: "session"
}

#[derive(Debug, Serialize, Deserialize, Model)]
pub struct Session {
	#[key]
	pub id: SessionId,

	pub user_id: UserId,
	pub refresh_token_hash: String,
	pub created_at: Timestamp,
	pub expires_at: Timestamp,
	pub revoked_at: Option<Timestamp>,
	pub refresh_expires_at: Timestamp,

	#[belongs_to]
	user: Deferred<User>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionClaims {
	pub session_id: SessionId,
	pub user_id: UserId,
	pub exp: i64,
	pub typ: String,
}
