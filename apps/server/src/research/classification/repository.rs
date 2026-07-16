use crate::research::*;
use crate::shared::{AppResult, Database};

use sqlx::{Postgres, QueryBuilder, Row};
use std::sync::Arc;
use sword::prelude::*;

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

	pub async fn find_topic_by_openalex_id(
		&self,
		openalex_id: &str,
	) -> AppResult<Option<ResearchTopic>> {
		sqlx::query_as::<_, ResearchTopic>("SELECT * FROM research_topics WHERE openalex_id = $1")
			.bind(openalex_id)
			.fetch_optional(self.database.pool())
			.await
			.map_err(Into::into)
	}

	pub async fn unknown_keyword_id(&self) -> AppResult<Option<ResearchKeyword>> {
		sqlx::query_as::<_, ResearchKeyword>("SELECT * FROM keywords WHERE openalex_id = 'unknown'")
			.fetch_optional(self.database.pool())
			.await
			.map_err(Into::into)
	}

	pub async fn upsert_keyword(
		&self,
		openalex_id: &str,
		name: &str,
	) -> AppResult<ResearchKeywordId> {
		let row = sqlx::query(
			"INSERT INTO keywords (openalex_id, name)
			VALUES ($1, $2) ON CONFLICT (openalex_id)
			DO UPDATE SET name = EXCLUDED.name RETURNING id",
		)
		.bind(openalex_id)
		.bind(name)
		.fetch_one(self.database.pool())
		.await?;
		Ok(ResearchKeywordId::from_uuid(row.get("id")))
	}

	pub async fn list_keywords(&self, f: ClassificationFilter) -> AppResult<Vec<ResearchKeyword>> {
		let mut query = QueryBuilder::<Postgres>::new("SELECT * FROM keywords WHERE 1=1");

		if let Some(search) = f.search {
			let pattern = format!("%{}%", search.trim());
			query.push(" AND name ILIKE ").push_bind(pattern);
		}

		query.push(" ORDER BY name");
		query.push(" LIMIT 50");

		query
			.build_query_as::<ResearchKeyword>()
			.fetch_all(self.database.pool())
			.await
			.map_err(Into::into)
	}

	pub async fn list_topics_by_work(&self, work_id: &WorkId) -> AppResult<Vec<ResearchTopicView>> {
		sqlx::query_as::<_, ResearchTopicView>(
			"SELECT
				wt.topic_id, t.name, wt.score,
				s.id AS subfield_id, s.name AS subfield_name,
				f.id AS field_id, f.name AS field_name,
				d.id AS domain_id, d.name AS domain_name
			FROM work_topics wt
			JOIN research_topics t ON t.id = wt.topic_id
			JOIN research_subfields s ON s.id = t.subfield_id
			JOIN research_fields f ON f.id = s.field_id
			JOIN research_domains d ON d.id = f.domain_id
			WHERE wt.work_id = $1
			ORDER BY wt.score DESC",
		)
		.bind(work_id)
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn list_keywords_by_work(
		&self,
		work_id: &WorkId,
	) -> AppResult<Vec<ResearchKeywordView>> {
		sqlx::query_as::<_, ResearchKeywordView>(
			"SELECT wk.keyword_id, k.name, wk.score
			FROM work_keywords wk
			JOIN keywords k ON k.id = wk.keyword_id
			WHERE wk.work_id = $1
			ORDER BY wk.score DESC",
		)
		.bind(work_id)
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}
}
