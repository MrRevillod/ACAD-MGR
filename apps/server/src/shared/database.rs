use crate::shared::AppResult;

use serde::Deserialize;
use std::sync::Arc;
use sword::prelude::*;
use toasty::{Db as Pool, models};

pub use toasty::Transaction as Tx;

#[injectable(provider)]
pub struct Database {
	pool: Arc<Pool>,
}

#[config(key = "postgres-db")]
#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
	pub user: String,
	pub password: String,
	pub database: String,
	pub port: String,
	pub host: String,
	pub migrations_path: String,
	pub min_connections: u8,
	pub max_connections: u8,
	pub acquire_timeout_ms: u64,
}

impl Database {
	pub async fn new(db_conf: DatabaseConfig) -> Self {
		let mut db = Pool::builder()
			.max_pool_size(db_conf.max_connections as usize)
			.models(models!(crate::*))
			.connect(&Self::create_uri(&db_conf))
			.await?;

		db.push_schema().expect("Failed to migrate database schema");

		let a = db.transaction().await?;

		Self { pool: Arc::new(db) }
	}

	fn create_uri(db_conf: &DatabaseConfig) -> String {
		format!(
			"postgres://{}:{}@{}:{}/{}",
			db_conf.user, db_conf.password, db_conf.host, db_conf.port, db_conf.database
		)
	}

	pub fn pool(&self) -> &Pool {
		&self.pool
	}

	pub async fn tx(&self) -> AppResult<Tx<'_>> {
		Ok(self.pool.transaction().await?)
	}
}

#[injectable]
pub struct TransactionManager {
	db: Arc<Database>,
}

impl TransactionManager {
	pub async fn begin(&self) -> AppResult<Tx<'_>> {
		self.db.tx().await
	}
}
