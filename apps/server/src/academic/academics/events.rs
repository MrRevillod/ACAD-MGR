use sword::prelude::event;

#[event(key = "academic.events.update-academic-form")]
pub struct UpdateAcademicFormEvent {
	pub academic_name: String,
	pub academic_email: String,
	pub form_url: String,
}

#[event(key = "academic.events.send-edit-codes")]
pub struct SendEditCodesEvent {
	pub academic_name: String,
	pub academic_email: String,
	pub codes: Vec<String>,
}
