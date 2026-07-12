mod careers;
mod countries;
mod departments;
mod errors;
mod faculties;
mod work_positions;

pub use careers::*;
pub use countries::*;
pub use departments::*;
pub use errors::*;
pub use faculties::*;
pub use work_positions::*;

use sword::prelude::*;

pub struct UniversityModule;

impl Module for UniversityModule {
	fn register_controllers(controllers: &ControllerRegistry) {
		controllers.register::<CareersController>();
		controllers.register::<DepartmentsController>();
		controllers.register::<FacultiesController>();
		controllers.register::<WorkPositionsController>();
	}

	fn register_components(components: &ComponentRegistry) {
		components.register::<FacultiesService>();
		components.register::<FacultiesRepository>();

		components.register::<DepartmentsService>();
		components.register::<DepartmentsRepository>();

		components.register::<CareersService>();
		components.register::<CareersRepository>();

		components.register::<AcademicWorkPositionsService>();
		components.register::<AcademicWorkPositionsRepository>();

		components.register::<CountriesRepository>();
	}
}
