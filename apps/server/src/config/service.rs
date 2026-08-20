use crate::config::*;
use crate::shared::AppResult;

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct ConfigService {
	repository: Arc<ConfigRepository>,
}

impl ConfigService {
	pub async fn get(&self) -> AppResult<AppConfig> {
		let Some(config) = self.repository.find().await? else {
			return Err(ConfigError::NotFound)?;
		};

		Ok(config)
	}

	pub async fn jce_max(&self) -> AppResult<f64> {
		Ok(self.get().await?.jce_max)
	}

	pub async fn update(&self, input: UpdateAppConfigDto) -> AppResult<AppConfig> {
		let config = AppConfig {
			jce_max: input.jce_max,
		};

		self.repository.update(&config).await?;

		Ok(config)
	}
}
