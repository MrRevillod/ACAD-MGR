use std::collections::HashMap;

pub struct TemplateRenderer;

impl TemplateRenderer {
	pub fn render(template_name: &str, context: &HashMap<String, String>) -> String {
		let mut template = match template_name {
			"academic-updater-form" => include_str!("academic-updater-form.html"),
			"sync-results" => include_str!("sync-results.html"),
			_ => panic!("Template not found"),
		}
		.to_string();

		for (key, value) in context {
			let placeholder = format!("{{{{{}}}}}", key);
			template = template.replace(&placeholder, value);
		}

		template
	}
}
