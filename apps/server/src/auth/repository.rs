use crate::{
	auth::{Session, SessionId},
	shared::{AppResult, Database},
};

use jiff::Timestamp;
use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct SessionRepository {
	database: Arc<Database>,
}

impl SessionRepository {
	pub async fn save(&self, session: &Session) -> AppResult<Session> {
		let session = Session::upsert_by_id(session.id)
			.user_id(session.id)
			.refresh_token_hash(session.refresh_token_hash.clone())
			.created_at(session.created_at)
			.expires_at(session.expires_at)
			.refresh_expires_at(session.refresh_expires_at)
			.revoked_at(session.revoked_at)
			.exec(&mut self.database.pool())
			.await?;

		Ok(session)
	}

	pub async fn is_active(&self, id: &SessionId) -> AppResult<bool> {
		let res = Session::filter_by_id(id)
			.filter(Session::fields().revoked_at().is_none())
			.filter(Session::fields().expires_at().gt(Timestamp::now()))
			.first()
			.exec(&mut self.database.pool())
			.await?;

		Ok(res.is_some())
	}

	pub async fn find_active_by_id(&self, id: &SessionId) -> AppResult<Option<Session>> {
		let res = Session::filter_by_id(id)
			.filter(Session::fields().revoked_at().is_none())
			.filter(Session::fields().expires_at().gt(Timestamp::now()))
			.first()
			.exec(&mut self.database.pool())
			.await?;

		Ok(res)
	}

	pub async fn find_active_by_refresh_id(&self, id: &SessionId) -> AppResult<Option<Session>> {
		let res = Session::filter_by_id(id)
			.filter(Session::fields().revoked_at().is_none())
			.filter(Session::fields().refresh_expires_at().gt(Timestamp::now()))
			.first()
			.exec(&mut self.database.pool())
			.await?;

		Ok(res)
	}

	pub async fn update_expires_at(&self, id: &SessionId, expires_at: Timestamp) -> AppResult<()> {
		Session::update_by_id(id)
			.expires_at(expires_at)
			.exec(&mut self.database.pool())
			.await?;

		Ok(())
	}
}
