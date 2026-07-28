use crate::{
	shared::{AppError, AppResult, Database},
	university::Country,
};

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct CountriesRepository {
	database: Arc<Database>,
}

impl CountriesRepository {
	pub async fn find_by_code(&self, code: &str) -> AppResult<Option<String>> {
		Country::filter_by_code(code)
			.select(Country::fields().code())
			.first()
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)
	}
}
