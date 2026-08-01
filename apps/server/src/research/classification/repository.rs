use crate::research::*;
use crate::shared::{AppResult, Database};

use sqlx::{Postgres, QueryBuilder};
use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct WorkClassificationRepository {
	database: Arc<Database>,
}

impl WorkClassificationRepository {
	pub async fn list_domains(&self, f: ClassificationFilter) -> AppResult<Vec<Domain>> {
		let mut query = QueryBuilder::<Postgres>::new("SELECT * FROM domains WHERE 1=1");

		if let Some(search) = f.search {
			let pattern = format!("%{}%", search.trim());
			query.push(" AND name ILIKE ").push_bind(pattern);
		}

		query.push(" ORDER BY name");

		query
			.build_query_as::<Domain>()
			.fetch_all(self.database.pool())
			.await
			.map_err(Into::into)
	}

	pub async fn list_fields(&self, f: ClassificationFilter) -> AppResult<Vec<Field>> {
		let mut query = QueryBuilder::<Postgres>::new("SELECT * FROM fields WHERE 1=1");

		if let Some(domain_id) = f.domain_id {
			query.push(" AND domain_id = ").push_bind(domain_id);
		}

		if let Some(search) = f.search {
			let pattern = format!("%{}%", search.trim());
			query.push(" AND name ILIKE ").push_bind(pattern);
		}

		query.push(" ORDER BY name");

		query
			.build_query_as::<Field>()
			.fetch_all(self.database.pool())
			.await
			.map_err(Into::into)
	}

	pub async fn list_subfields(&self, f: ClassificationFilter) -> AppResult<Vec<Subfield>> {
		let mut query = QueryBuilder::<Postgres>::new("SELECT * FROM subfields WHERE 1=1");

		if let Some(field_id) = f.field_id {
			query.push(" AND field_id = ").push_bind(field_id);
		}

		if let Some(search) = f.search {
			let pattern = format!("%{}%", search.trim());
			query.push(" AND name ILIKE ").push_bind(pattern);
		}

		query.push(" ORDER BY name");
		query.push(" LIMIT ").push_bind(f.limit.unwrap_or(50));

		query
			.build_query_as::<Subfield>()
			.fetch_all(self.database.pool())
			.await
			.map_err(Into::into)
	}

	pub async fn list_topics(&self, f: ClassificationFilter) -> AppResult<Vec<Topic>> {
		let mut query = QueryBuilder::<Postgres>::new("SELECT * FROM topics WHERE 1=1");

		if let Some(subfield_id) = f.subfield_id {
			query.push(" AND subfield_id = ").push_bind(subfield_id);
		}

		if let Some(search) = f.search {
			let pattern = format!("%{}%", search.trim());
			query.push(" AND name ILIKE ").push_bind(pattern);
		}

		query.push(" ORDER BY name");
		query.push(" LIMIT 50");

		query
			.build_query_as::<Topic>()
			.fetch_all(self.database.pool())
			.await
			.map_err(Into::into)
	}

	pub async fn unknown_topic_id(&self) -> AppResult<Option<Topic>> {
		let topic =
			sqlx::query_as::<_, Topic>("SELECT * FROM topics WHERE openalex_id = 'unknown'")
				.fetch_optional(self.database.pool())
				.await?;

		Ok(topic)
	}

	pub async fn find_topic_by_openalex_id(&self, openalex_id: &str) -> AppResult<Option<Topic>> {
		sqlx::query_as::<_, Topic>("SELECT * FROM topics WHERE openalex_id = $1")
			.bind(openalex_id)
			.fetch_optional(self.database.pool())
			.await
			.map_err(Into::into)
	}

	pub async fn unknown_keyword_id(&self) -> AppResult<Option<Keyword>> {
		sqlx::query_as::<_, Keyword>("SELECT * FROM keywords WHERE openalex_id = 'unknown'")
			.fetch_optional(self.database.pool())
			.await
			.map_err(Into::into)
	}

	pub async fn find_keyword_by_openalex_id(
		&self,
		openalex_id: &str,
	) -> AppResult<Option<Keyword>> {
		sqlx::query_as::<_, Keyword>("SELECT * FROM keywords WHERE openalex_id = $1")
			.bind(openalex_id)
			.fetch_optional(self.database.pool())
			.await
			.map_err(Into::into)
	}

	pub async fn save_keyword(&self, keyword: &Keyword) -> AppResult<()> {
		sqlx::query(
			"INSERT INTO keywords (id, openalex_id, name)
			VALUES ($1, $2, $3)
			ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name",
		)
		.bind(keyword.id)
		.bind(&keyword.openalex_id)
		.bind(&keyword.name)
		.execute(self.database.pool())
		.await?;

		Ok(())
	}

	pub async fn find_subfield_by_openalex_id(
		&self,
		openalex_id: &str,
	) -> AppResult<Option<Subfield>> {
		sqlx::query_as::<_, Subfield>("SELECT * FROM subfields WHERE openalex_id = $1")
			.bind(openalex_id)
			.fetch_optional(self.database.pool())
			.await
			.map_err(Into::into)
	}

	pub async fn save_subfield(&self, subfield: &Subfield) -> AppResult<()> {
		sqlx::query(
			"INSERT INTO subfields (id, openalex_id, name, field_id, research_line_id)
			VALUES ($1, $2, $3, $4, $5)
			ON CONFLICT (id) DO UPDATE SET
				name = EXCLUDED.name,
				field_id = EXCLUDED.field_id,
				research_line_id = EXCLUDED.research_line_id",
		)
		.bind(subfield.id)
		.bind(&subfield.openalex_id)
		.bind(&subfield.name)
		.bind(subfield.field_id)
		.bind(subfield.research_line_id)
		.execute(self.database.pool())
		.await?;

		Ok(())
	}

	pub async fn list_keywords(&self, f: ClassificationFilter) -> AppResult<Vec<Keyword>> {
		let mut query = QueryBuilder::<Postgres>::new("SELECT * FROM keywords WHERE 1=1");

		if let Some(search) = f.search {
			let pattern = format!("%{}%", search.trim());
			query.push(" AND name ILIKE ").push_bind(pattern);
		}

		query.push(" ORDER BY name");
		query.push(" LIMIT 50");

		query
			.build_query_as::<Keyword>()
			.fetch_all(self.database.pool())
			.await
			.map_err(Into::into)
	}

	pub async fn list_research_lines(&self) -> AppResult<Vec<ResearchLine>> {
		sqlx::query_as::<_, ResearchLine>("SELECT id, name, slug FROM research_lines ORDER BY name")
			.fetch_all(self.database.pool())
			.await
			.map_err(Into::into)
	}
}
