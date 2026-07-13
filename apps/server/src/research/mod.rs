mod authorships;
mod classification;
mod sources;
mod works;

pub use authorships::*;
pub use classification::*;
pub use sources::*;
pub use works::*;

use sword::prelude::*;

pub struct ResearchModule;

impl Module for ResearchModule {
	fn register_controllers(controllers: &ControllerRegistry) {
		controllers.register::<WorksClassificationController>();
		controllers.register::<WorksController>();
	}

	fn register_components(components: &ComponentRegistry) {
		components.register::<WorkClassificationRepository>();
		components.register::<WorksService>();
		components.register::<WorksRepository>();
		components.register::<SourcesRepository>();
		components.register::<AuthorshipsRepository>();
	}

	async fn register_providers(config: &Config, providers: &ProviderRegistry) {
		let config = config.expect::<OpenAlexConfig>();
		let client = OpenAlexClient::new(config);

		providers.register(client);
	}
}
