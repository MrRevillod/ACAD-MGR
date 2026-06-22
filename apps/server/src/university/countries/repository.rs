use crate::shared::{AppResult, Database};

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct CountriesRepository {
    database: Arc<Database>,
}

impl CountriesRepository {
    pub async fn find_by_name(&self, name: &str) -> AppResult<Option<String>> {
        let item = sqlx::query_scalar::<_, String>(
            "SELECT code FROM countries WHERE name = $1",
        )
        .bind(name)
        .fetch_optional(self.database.pool())
        .await?;

        Ok(item)
    }
}
