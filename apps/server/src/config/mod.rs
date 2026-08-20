mod controller;
mod dtos;
mod entity;
mod errors;
mod repository;
mod service;

pub use controller::*;
pub use dtos::*;
pub use entity::*;
pub use errors::*;
pub use repository::*;
pub use service::*;

use sword::prelude::*;

pub struct ConfigModule;

impl Module for ConfigModule {
	fn register_controllers(controllers: &ControllerRegistry) {
		controllers.register::<ConfigController>();
	}

	fn register_components(components: &ComponentRegistry) {
		components.register::<ConfigService>();
		components.register::<ConfigRepository>();
	}
}
