mod database;
mod errors;
mod extensions;
mod id;
mod jsonwebtoken;
mod logger;
mod seeder;

mod value_objects {
	mod cl_float;
	pub use cl_float::CLf64;

	mod country;
	pub use country::Country;
}

use std::sync::Arc;

use database::DatabaseConfig;
use sword::prelude::*;

pub use database::{Database, TransactionManager, Tx};
pub use errors::*;
pub use extensions::*;
pub use id::{Entity, Id};
pub use jsonwebtoken::JsonWebTokenService;
pub use logger::LoggerLayer;
pub use value_objects::CLf64;
pub use value_objects::Country;

use crate::shared::seeder::DatabaseSeeder;
use crate::shared::seeder::SeederData;

pub struct SharedModule;

impl Module for SharedModule {
	async fn register_providers(config: &Config, providers: &ProviderRegistry) {
		let db_config = config.expect::<DatabaseConfig>();
		let database = Database::new(db_config).await;

		let seeder_data = config.expect::<SeederData>();
		let seeder = DatabaseSeeder::new(Arc::new(database.clone()), seeder_data);

		seeder.seed().await;

		providers.register(database);
	}

	fn register_components(components: &ComponentRegistry) {
		components.register::<TransactionManager>();
		components.register::<JsonWebTokenService>();
	}
}
