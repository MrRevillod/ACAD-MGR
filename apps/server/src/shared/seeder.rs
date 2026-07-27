use crate::{
	auth::{User, UserId, UserRole},
	shared::Database,
};

use serde::Deserialize;
use sword::prelude::*;

#[derive(Debug, Clone, Deserialize)]
#[config(key = "seeder")]
pub struct SeederData {
	admin_email: String,
	admin_password_hash: String,
}

#[injectable(provider)]
pub struct DatabaseSeeder {
	database: Database,
	config: SeederData,
}

impl DatabaseSeeder {
	pub fn new(db_ref: Database, config: SeederData) -> Self {
		Self {
			database: db_ref,
			config,
		}
	}

	pub async fn seed(&self) {
		User::create()
			.id(UserId::new())
			.name("ADMINISTRACIÓN".to_string())
			.email(self.config.admin_email.clone())
			.password_hash(self.config.admin_password_hash.clone())
			.role(UserRole::Admin)
			.exec(&mut self.database.pool())
			.await
			.expect("Failed to seed admin user");

		tracing::info!("Database seeding completed successfully.");
	}
}
