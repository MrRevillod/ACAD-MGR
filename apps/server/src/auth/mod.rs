pub mod administrators;

pub use administrators::*;

use sword::prelude::*;

pub struct AuthModule;

impl Module for AuthModule {
    fn register_controllers(_controllers: &ControllerRegistry) {}

    fn register_components(components: &ComponentRegistry) {
        components.register::<administrators::AdministratorsRepository>();
    }
}
