pub mod academics;
pub mod categories;
pub mod degrees;
pub mod options;

pub use academics::*;
pub use categories::*;
pub use degrees::*;
pub use options::*;

use sword::prelude::*;

pub struct AcademicModule;

impl Module for AcademicModule {
    fn register_controllers(_controllers: &ControllerRegistry) {}

    fn register_components(components: &ComponentRegistry) {
        components.register::<AcademicCategoriesRepository>();
        components.register::<AcademicCategoryOptionsRepository>();
        components.register::<AcademicsRepository>();
        components.register::<DegreesRepository>();
    }
}
