pub mod classification;
pub mod works;

use sword::prelude::*;

use classification::*;
use works::{OpenAlexClient, OpenAlexConfig};
use works::{WorksController, WorksRepository, WorksService};

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
	}

	async fn register_providers(config: &Config, providers: &ProviderRegistry) {
		let config = config.expect::<OpenAlexConfig>();
		let client = OpenAlexClient::new(config);

		providers.register(client);
	}
}
