use std::collections::HashMap;
use std::sync::Arc;

use crate::research::{SyncWorksRequest, WorksService};
use crate::shared::{Mail, Mailer, TemplateRenderer};

use sword::events::*;
use sword::prelude::*;

#[controller(kind = Controller::MemEventHandler, namespace = "works")]
pub struct WorksEventsController {
	mailer: Arc<Mailer>,
	works: Arc<WorksService>,
}

impl WorksEventsController {
	#[handle("sync-requested")]
	async fn handle_sync_requested(&self, event: SyncWorksRequest) -> EventHandlerResult<()> {
		tracing::info!("processing sync-requested event for {}", event.user_email);
		let result = self.works.sync_all_academics().await;

		match result {
			Ok(results) => {
				let total = results.len();
				let error_orcids: Vec<&str> = results
					.iter()
					.filter(|r| !r.errors.is_empty())
					.map(|r| r.academic_orcid.as_str())
					.collect();

				tracing::info!(
					"sync completed: {} academics, {} errors",
					total,
					error_orcids.len()
				);

				let (status_suffix, error_details, subject) = if error_orcids.is_empty() {
					(
						" exitosamente".to_string(),
						String::new(),
						"Sincronización completada exitosamente",
					)
				} else {
					let n = error_orcids.len();
					let items: String =
						error_orcids.iter().map(|o| format!("<li>{}</li>", o)).collect();
					(
						format!(" con {n} académico(s) con errores"),
						format!(
							"<p style=\"margin:0;padding:0;font-size:1em;padding-top:0.5em;padding-bottom:0.5em;color:#b91c1c\">Académicos con errores:</p><ul style=\"font-size:0.875em;color:#555\">{items}</ul>"
						),
						"Sincronización completada con errores",
					)
				};

				let context = HashMap::from([
					("STATUS_SUFFIX".into(), status_suffix),
					("ERROR_DETAILS".into(), error_details),
				]);
				let html = TemplateRenderer::render("sync-results", &context);

				let mail = Mail::builder()
					.to(event.user_email)
					.subject(subject.into())
					.html(html)
					.build();

				self.mailer.send(mail).await.ok();
			}
			Err(e) => {
				tracing::error!("sync failed: {e}");
				let html = format!("<p>La sincronización falló: {e}</p>");
				let mail = Mail::builder()
					.to(event.user_email)
					.subject("Error en sincronización de publicaciones".into())
					.html(html)
					.build();

				self.mailer.send(mail).await.ok();
			}
		}

		Ok(())
	}
}
