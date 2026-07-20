use crate::academic::UpdateAcademicFormEvent;
use crate::shared::*;

use std::collections::HashMap;
use std::sync::Arc;
use sword::events::*;
use sword::prelude::*;

#[controller(kind = Controller::MemEventHandler, namespace = "academic.events")]
pub struct AcademicEventsController {
	mailer: Arc<Mailer>,
}

impl AcademicEventsController {
	#[handle("update-academic-form")]
	async fn update_academic_form(&self, e: UpdateAcademicFormEvent) -> EventHandlerResult<()> {
		let template_variables = HashMap::from([
			("ACADEMIC_NAME".to_string(), e.academic_name),
			("FORM_URL".to_string(), e.form_url),
		]);

		let template = TemplateRenderer::render("academic-updater-form", &template_variables);

		let mail = Mail::builder()
			.to(e.academic_email)
			.subject("Solicitud de actualización de perfil académico".into())
			.html(template)
			.build();

		self.mailer.send(mail).await.ok();

		Ok(())
	}
}
