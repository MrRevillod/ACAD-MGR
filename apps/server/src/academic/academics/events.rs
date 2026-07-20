use sword::prelude::event;

#[event(key = "academic.events.update-academic-form")]
pub struct UpdateAcademicFormEvent {
	pub academic_name: String,
	pub academic_email: String,
	pub form_url: String,
}
