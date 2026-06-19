pub mod faculties;
pub mod departments;
pub mod careers;
pub mod work_positions;

pub use faculties::*;
pub use departments::*;
pub use careers::*;
pub use work_positions::*;

use sword::prelude::*;

pub struct UniversityModule;

impl Module for UniversityModule {
    fn register_controllers(_controllers: &ControllerRegistry) {}

    fn register_components(components: &ComponentRegistry) {
        components.register::<faculties::FacultiesRepository>();
        components.register::<departments::DepartmentsRepository>();
        components.register::<careers::CareersRepository>();
        components.register::<work_positions::AcademicWorkPositionsRepository>();
    }
}
