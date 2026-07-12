use crate::research::classification::*;
use crate::shared::{AppResult, Database};

use sqlx::{Postgres, QueryBuilder};
use std::sync::Arc;
use sword::prelude::*;

pub struct ClassificationFilter {
	pub domain_id: Option<ResearchDomainId>,
	pub field_id: Option<ResearchFieldId>,
	pub subfield_id: Option<ResearchSubfieldId>,
	pub topic_id: Option<ResearchTopicId>,
	pub openalex_id: Option<String>,
	pub search: Option<String>,
}

#[injectable]
pub struct WorkClassificationRepository {
	database: Arc<Database>,
}

impl WorkClassificationRepository {
	pub async fn list_domains(&self, f: ClassificationFilter) -> AppResult<Vec<ResearchDomain>> {
		let mut query = QueryBuilder::<Postgres>::new("SELECT * FROM research_domains WHERE 1=1");

		if let Some(search) = f.search {
			let pattern = format!("%{}%", search.trim());
			query.push(" AND name ILIKE ").push_bind(pattern);
		}

		query.push(" ORDER BY name");

		query
			.build_query_as::<ResearchDomain>()
			.fetch_all(self.database.pool())
			.await
			.map_err(Into::into)
	}

	pub async fn list_fields(&self, f: ClassificationFilter) -> AppResult<Vec<ResearchField>> {
		let mut query = QueryBuilder::<Postgres>::new("SELECT * FROM research_fields WHERE 1=1");

		if let Some(domain_id) = f.domain_id {
			query.push(" AND domain_id = ").push_bind(domain_id);
		}

		if let Some(search) = f.search {
			let pattern = format!("%{}%", search.trim());
			query.push(" AND name ILIKE ").push_bind(pattern);
		}

		query.push(" ORDER BY name");

		query
			.build_query_as::<ResearchField>()
			.fetch_all(self.database.pool())
			.await
			.map_err(Into::into)
	}

	pub async fn list_subfields(
		&self,
		f: ClassificationFilter,
	) -> AppResult<Vec<ResearchSubfield>> {
		let mut query = QueryBuilder::<Postgres>::new("SELECT * FROM research_subfields WHERE 1=1");

		if let Some(field_id) = f.field_id {
			query.push(" AND field_id = ").push_bind(field_id);
		}

		if let Some(search) = f.search {
			let pattern = format!("%{}%", search.trim());
			query.push(" AND name ILIKE ").push_bind(pattern);
		}

		query.push(" ORDER BY name");
		query.push(" LIMIT 50");

		query
			.build_query_as::<ResearchSubfield>()
			.fetch_all(self.database.pool())
			.await
			.map_err(Into::into)
	}

	pub async fn list_topics(&self, f: ClassificationFilter) -> AppResult<Vec<ResearchTopic>> {
		let mut query = QueryBuilder::<Postgres>::new("SELECT * FROM research_topics WHERE 1=1");

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
			.build_query_as::<ResearchTopic>()
			.fetch_all(self.database.pool())
			.await
			.map_err(Into::into)
	}

	pub async fn unknown_topic_id(&self) -> AppResult<Option<ResearchTopic>> {
		let topic = sqlx::query_as::<_, ResearchTopic>(
			"SELECT * FROM research_topics WHERE openalex_id = 'unknown'",
		)
		.fetch_optional(self.database.pool())
		.await?;

		Ok(topic)
	}

	pub async fn list_keywords(&self, f: ClassificationFilter) -> AppResult<Vec<ResearchKeyword>> {
		let mut query = QueryBuilder::<Postgres>::new("SELECT * FROM research_keywords WHERE 1=1");

		if let Some(search) = f.search {
			let pattern = format!("%{}%", search.trim());
			query.push(" AND keyword ILIKE ").push_bind(pattern);
		}

		query.push(" ORDER BY keyword");
		query.push(" LIMIT 50");

		query
			.build_query_as::<ResearchKeyword>()
			.fetch_all(self.database.pool())
			.await
			.map_err(Into::into)
	}
}
