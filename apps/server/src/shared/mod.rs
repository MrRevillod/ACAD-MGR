mod cookies;
mod countries;
mod database;
mod errors;
mod extensions;
mod id;
mod jsonwebtoken;

use database::DatabaseConfig;
use sword::prelude::*;

pub use cookies::*;
pub use countries::*;
pub use database::{Database, TransactionManager, Tx};
pub use errors::*;
pub use extensions::*;
pub use id::{Entity, Id};

pub struct SharedModule;

impl Module for SharedModule {
    async fn register_providers(config: &Config, providers: &ProviderRegistry) {
        let db_config = config.expect::<DatabaseConfig>();
        let database = Database::new(db_config).await;

        providers.register(database);
    }

    fn register_components(components: &ComponentRegistry) {
        components.register::<TransactionManager>();
    }
}
