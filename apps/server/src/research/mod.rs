pub mod classification;
pub mod works;

use classification::WorksClassificationController;
use classification::*;
use sword::prelude::*;
use works::{OpenAlexClient, OpenAlexConfig};
use works::{WorksController, WorksRepository, WorksService};

pub struct ResearchModule;

impl Module for ResearchModule {
    fn register_controllers(controllers: &ControllerRegistry) {
        controllers.register::<WorksClassificationController>();
        controllers.register::<WorksController>();
    }

    fn register_components(components: &ComponentRegistry) {
        components.register::<ResearchDomainsService>();
        components.register::<ResearchDomainsRepository>();
        components.register::<ResearchFieldsService>();
        components.register::<ResearchFieldsRepository>();
        components.register::<ResearchSubfieldsService>();
        components.register::<ResearchSubfieldsRepository>();
        components.register::<ResearchTopicsService>();
        components.register::<ResearchTopicsRepository>();
        components.register::<ResearchKeywordsService>();
        components.register::<ResearchKeywordsRepository>();

        components.register::<WorksService>();
        components.register::<WorksRepository>();
    }

    async fn register_providers(config: &Config, providers: &ProviderRegistry) {
        let config = config.expect::<OpenAlexConfig>();

        providers.register(OpenAlexClient::new(config));
    }
}
