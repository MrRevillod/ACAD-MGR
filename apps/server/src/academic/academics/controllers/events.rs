use crate::academic::{SendEditCodesEvent, UpdateAcademicFormEvent};
use crate::shared::*;

use std::collections::HashMap;
use std::sync::Arc;
use sword::events::*;
use sword::prelude::*;

#[controller(kind = Controller::EventHandler, source = EventSource::Memory)]
pub struct AcademicEventsController {
	mailer: Arc<Mailer>,
}

impl AcademicEventsController {
	#[handle("academic.events.update-academic-form")]
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

	#[handle("academic.events.send-edit-codes")]
	async fn send_edit_codes(&self, e: SendEditCodesEvent) -> EventHandlerResult<()> {
		let codes_html = e
			.codes
			.iter()
			.map(|code| {
				format!(
					"<div style=\"margin:4px 0;padding:8px 12px;border:1px solid #cbd5e1;border-radius:6px;font-family:monospace;font-size:1.05em;letter-spacing:1px;background-color:#f8fafc\">{code}</div>"
				)
			})
			.collect::<Vec<_>>()
			.join("");

		let template_variables = HashMap::from([
			("ACADEMIC_NAME".to_string(), e.academic_name),
			("CODES".to_string(), codes_html),
		]);

		let template = TemplateRenderer::render("academic-edit-codes", &template_variables);

		let mail = Mail::builder()
			.to(e.academic_email)
			.subject("Códigos de autorización para edición de perfil".into())
			.html(template)
			.build();

		self.mailer.send(mail).await.ok();

		Ok(())
	}
}
