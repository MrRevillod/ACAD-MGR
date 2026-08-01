mod authorships;
mod classification;
mod collaborations;
mod sources;
mod stats;
mod works;

pub use authorships::*;
pub use classification::*;
pub use collaborations::*;
pub use sources::*;
pub use stats::*;
pub use works::*;

use sword::prelude::*;

pub struct ResearchModule;

impl Module for ResearchModule {
	fn register_controllers(controllers: &ControllerRegistry) {
		controllers.register::<StatsController>();
		controllers.register::<WorksClassificationController>();

		controllers.register::<WorksController>();
		controllers.register::<WorksEventsController>();

		controllers.register::<CollaborationsController>();
	}

	fn register_components(components: &ComponentRegistry) {
		components.register::<StatsService>();
		components.register::<StatsRepository>();
		components.register::<WorkClassificationRepository>();
		components.register::<WorksService>();
		components.register::<WorksImportService>();
		components.register::<WorksRepository>();
		components.register::<SourcesRepository>();
		components.register::<AuthorshipsRepository>();
		components.register::<CollaborationsService>();
		components.register::<CollaborationsRepository>();
	}

	async fn register_providers(config: &Config, providers: &ProviderRegistry) {
		let config = config.expect::<OpenAlexConfig>();
		let client = OpenAlexClient::new(config);

		providers.register(client);
	}
}
