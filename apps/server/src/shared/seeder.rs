use crate::{
	auth::{User, UserId, UserRole},
	shared::Database,
};

use serde::Deserialize;
use std::sync::Arc;
use sword::prelude::*;

#[derive(Debug, Clone, Deserialize)]
#[config(key = "seeder")]
pub struct SeederData {
	admin_email: String,
	admin_password_hash: String,
}

#[injectable(provider)]
pub struct DatabaseSeeder {
	database: Arc<Database>,
	config: SeederData,
}

impl DatabaseSeeder {
	pub fn new(db_ref: Arc<Database>, config: SeederData) -> Self {
		Self {
			database: db_ref,
			config,
		}
	}

	pub async fn seed(&self) {
		let admin = User {
			id: UserId::new(),
			name: "ADMINISTRACIÓN".to_string(),
			email: self.config.admin_email.clone(),
			password_hash: self.config.admin_password_hash.clone(),
			role: UserRole::Admin,
		};

		sqlx::query(
			r"
        	INSERT INTO users (id, name, email, password_hash, role)
         	VALUES ($1, $2, $3, $4, $5)
          	ON CONFLICT (email) DO NOTHING
        ",
		)
		.bind(admin.id)
		.bind(admin.name)
		.bind(admin.email)
		.bind(admin.password_hash)
		.bind(admin.role)
		.execute(self.database.pool())
		.await
		.expect("Failed to seed admin user");

		tracing::info!("Database seeding completed successfully.");
	}
}
