use crate::config::AppConfig;
use crate::shared::{AppResult, Database};

use sqlx::types::Json;
use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct ConfigRepository {
	database: Arc<Database>,
}

impl ConfigRepository {
	pub async fn find(&self) -> AppResult<Option<AppConfig>> {
		let config =
			sqlx::query_scalar::<_, Json<AppConfig>>("SELECT data FROM app_config WHERE id = 1")
				.fetch_optional(self.database.pool())
				.await?
				.map(|Json(config)| config);

		Ok(config)
	}

	pub async fn update(&self, config: &AppConfig) -> AppResult<()> {
		sqlx::query("UPDATE app_config SET data = $1 WHERE id = 1")
			.bind(Json(config))
			.execute(self.database.pool())
			.await?;

		Ok(())
	}
}
