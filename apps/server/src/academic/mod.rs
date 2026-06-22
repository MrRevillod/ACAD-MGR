pub mod academics;
pub mod categories;
pub mod degrees;
pub mod errors;
pub mod options;

pub use academics::*;
pub use categories::*;
pub use degrees::*;
pub use errors::*;
pub use options::*;

use sword::prelude::*;

pub struct AcademicModule;

impl Module for AcademicModule {
    fn register_controllers(controllers: &ControllerRegistry) {
        controllers.register::<AcademicCategoriesController>();
        controllers.register::<AcademicCategoryOptionsController>();
        controllers.register::<DegreesController>();
        controllers.register::<AcademicsController>();
    }

    fn register_components(components: &ComponentRegistry) {
        components.register::<AcademicCategoriesService>();
        components.register::<AcademicCategoriesRepository>();

        components.register::<AcademicCategoryOptionsService>();
        components.register::<AcademicCategoryOptionsRepository>();

        components.register::<DegreesService>();
        components.register::<DegreesRepository>();

        components.register::<AcademicsService>();
        components.register::<AcademicsRepository>();

        components.register::<ImportsService>();
    }
}
