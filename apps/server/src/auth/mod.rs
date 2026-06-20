mod controller;
mod dtos;
mod entity;
mod errors;
mod interceptor;
mod repository;
mod services;
mod users;

pub use dtos::*;
pub use entity::*;
pub use errors::*;
pub use interceptor::*;
pub use repository::*;
pub use services::*;
pub use users::*;

use serde::Deserialize;
use sword::prelude::*;

#[config(key = "auth")]
#[derive(Clone, Deserialize)]
pub struct AuthConfig {
    pub access_exp_minutes: i64,
    pub refresh_exp_days: i64,
    pub jwt_secret: String,
}

pub struct AuthModule;

impl Module for AuthModule {
    fn register_controllers(_: &ControllerRegistry) {}

    fn register_components(components: &ComponentRegistry) {
        components.register::<UsersRepository>();
    }

    async fn register_providers(config: &Config, providers: &ProviderRegistry) {
        let hasher_config = config.expect::<HasherConfig>();
        let hasher_provider = Hasher::new(&hasher_config);

        providers.register::<Hasher>(hasher_provider);
    }
}
