pub mod categories;
pub mod options;
pub mod academics;
pub mod degrees;

pub use categories::*;
pub use options::*;
pub use academics::*;
pub use degrees::*;

use sword::prelude::*;

pub struct AcademicModule;

impl Module for AcademicModule {
    fn register_controllers(_controllers: &ControllerRegistry) {}

    fn register_components(components: &ComponentRegistry) {
        components.register::<categories::AcademicCategoriesRepository>();
        components.register::<options::AcademicCategoryOptionsRepository>();
        components.register::<academics::AcademicsRepository>();
        components.register::<degrees::DegreesRepository>();
    }
}
