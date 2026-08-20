use crate::auth::SessionCheck;
use crate::config::{AppConfigResponse, ConfigService, UpdateAppConfigDto};

use std::sync::Arc;
use sword::prelude::*;
use sword::web::*;

#[controller(kind = ControllerKind::Web, path = "/config")]
#[interceptor(SessionCheck)]
pub struct ConfigController {
	service: Arc<ConfigService>,
}

impl ConfigController {
	#[get("/")]
	pub async fn get_config(&self) -> WebResult<AppConfigResponse> {
		Ok(self.service.get().await?.into())
	}

	#[patch("/")]
	pub async fn update_config(&self, req: Request) -> WebResult<AppConfigResponse> {
		let input = req.body_validator::<UpdateAppConfigDto>()?;
		Ok(self.service.update(input).await?.into())
	}
}
