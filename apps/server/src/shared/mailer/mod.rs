mod templates;

pub use templates::TemplateRenderer;

use bon::Builder;
use lettre::{
	AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
	message::{Mailbox, header::ContentType},
	transport::smtp::authentication::Credentials,
};

use serde::Deserialize;
use sword::prelude::*;

#[config(key = "mailer")]
#[derive(Debug, Clone, Deserialize)]
pub struct MailerConfig {
	pub smtp_host: String,
	pub smtp_port: String,
	pub smtp_username: String,
	pub smtp_password: String,
}

#[derive(Debug, Clone, Builder)]
pub struct Mail {
	pub to: String,
	pub subject: String,
	pub html: String,
}

#[injectable(provider)]
pub struct Mailer {
	config: MailerConfig,
	client: AsyncSmtpTransport<Tokio1Executor>,
}

impl Mailer {
	pub fn new(config: MailerConfig) -> Self {
		let credentials =
			Credentials::new(config.smtp_username.clone(), config.smtp_password.clone());

		let client = AsyncSmtpTransport::<Tokio1Executor>::relay(&config.smtp_host)
			.expect("Failed to create SMTP transport")
			.port(config.smtp_port.parse().expect("Invalid SMTP port"))
			.credentials(credentials)
			.build();

		Self { client, config }
	}

	pub async fn send(&self, mail: Mail) -> Result<(), Box<dyn std::error::Error>> {
		let email_from_fmt = format!(
			"Sistema de publicaciones académicas<{}>",
			self.config.smtp_username
		);

		let from = email_from_fmt
			.parse::<Mailbox>()
			.inspect_err(|e| eprintln!("Failed to parse sender email address: {e}"))?;

		let to = mail
			.to
			.parse::<Mailbox>()
			.inspect_err(|e| eprintln!("Failed to parse recipient email address: {e}"))?;

		let message = Message::builder()
			.from(from)
			.to(to)
			.subject(mail.subject)
			.header(ContentType::TEXT_HTML)
			.body(mail.html.clone())
			.inspect_err(|e| eprintln!("Failed to build email message: {e}"))?;

		self.client.send(message).await.inspect_err(|e| {
			eprintln!("Failed to send email: {e}");
		})?;

		Ok(())
	}
}
