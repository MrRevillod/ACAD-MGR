use crate::research::*;
use crate::shared::{AppResult, Database};

use sqlx::{Postgres, QueryBuilder, Row};
use std::sync::Arc;
use sword::prelude::*;
use uuid::Uuid;

#[injectable]
pub struct WorkClassificationRepository {
	database: Arc<Database>,
}

impl WorkClassificationRepository {
	pub async fn list_domains(&self, f: ClassificationFilter) -> AppResult<Vec<ResearchDomain>> {
		let mut query = QueryBuilder::<Postgres>::new("SELECT * FROM domains WHERE 1=1");

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
			.build_query_as::<ResearchField>()
			.fetch_all(self.database.pool())
			.await
			.map_err(Into::into)
	}

	pub async fn list_subfields(
		&self,
		f: ClassificationFilter,
	) -> AppResult<Vec<ResearchSubfield>> {
		let mut query = QueryBuilder::<Postgres>::new("SELECT * FROM subfields WHERE 1=1");

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
			.build_query_as::<ResearchTopic>()
			.fetch_all(self.database.pool())
			.await
			.map_err(Into::into)
	}

	pub async fn unknown_topic_id(&self) -> AppResult<Option<ResearchTopic>> {
		let topic = sqlx::query_as::<_, ResearchTopic>(
			"SELECT * FROM topics WHERE openalex_id = 'unknown'",
		)
		.fetch_optional(self.database.pool())
		.await?;

		Ok(topic)
	}

	pub async fn find_topic_by_openalex_id(
		&self,
		openalex_id: &str,
	) -> AppResult<Option<ResearchTopic>> {
		sqlx::query_as::<_, ResearchTopic>("SELECT * FROM topics WHERE openalex_id = $1")
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

	pub async fn find_keyword_by_openalex_id(
		&self,
		openalex_id: &str,
	) -> AppResult<Option<ResearchKeyword>> {
		sqlx::query_as::<_, ResearchKeyword>("SELECT * FROM keywords WHERE openalex_id = $1")
			.bind(openalex_id)
			.fetch_optional(self.database.pool())
			.await
			.map_err(Into::into)
	}

	pub async fn save_keyword(&self, keyword: &ResearchKeyword) -> AppResult<()> {
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
	) -> AppResult<Option<ResearchSubfield>> {
		sqlx::query_as::<_, ResearchSubfield>("SELECT * FROM subfields WHERE openalex_id = $1")
			.bind(openalex_id)
			.fetch_optional(self.database.pool())
			.await
			.map_err(Into::into)
	}

	pub async fn save_subfield(&self, subfield: &ResearchSubfield) -> AppResult<()> {
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

	pub async fn list_research_lines(&self) -> AppResult<Vec<ResearchLineView>> {
		sqlx::query_as::<_, ResearchLineView>(
			"SELECT id, name, slug FROM research_lines ORDER BY name",
		)
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}

	pub async fn list_research_lines_with_subfields(&self) -> AppResult<Vec<ResearchLineDetail>> {
		let rows = sqlx::query(
			"SELECT rl.id, rl.name, rl.slug,
				COALESCE(jsonb_agg(
					jsonb_build_object(
						'subfieldOpenalexId', rs.openalex_id,
						'subfieldName', rs.name
					)
					ORDER BY rs.name
				) FILTER (WHERE rs.openalex_id IS NOT NULL), '[]'::jsonb) AS subfields
			FROM research_lines rl
			LEFT JOIN subfields rs ON rs.research_line_id = rl.id
			GROUP BY rl.id, rl.name, rl.slug
			ORDER BY rl.name",
		)
		.fetch_all(self.database.pool())
		.await?;

		let lines = rows
			.into_iter()
			.map(|row| {
				let id: Uuid = row.get("id");
				let name: String = row.get("name");
				let slug: String = row.get("slug");
				let subfields: serde_json::Value = row.get("subfields");
				let subfields: Vec<SubfieldMapping> =
					serde_json::from_value(subfields).unwrap_or_default();
				ResearchLineDetail {
					id,
					name,
					slug,
					subfields,
				}
			})
			.collect();

		Ok(lines)
	}



	pub async fn list_topics_by_work(&self, work_id: &WorkId) -> AppResult<Vec<ResearchTopicView>> {
		sqlx::query_as::<_, ResearchTopicView>(
			"SELECT
				wt.topic_id, t.name, wt.score,
				s.id AS subfield_id, s.name AS subfield_name,
				f.id AS field_id, f.name AS field_name,
				d.id AS domain_id, d.name AS domain_name
			FROM work_topic_scores wt
			JOIN topics t ON t.id = wt.topic_id
			JOIN subfields s ON s.id = t.subfield_id
			JOIN fields f ON f.id = s.field_id
			JOIN domains d ON d.id = f.domain_id
			WHERE wt.work_id = $1
			ORDER BY wt.score DESC",
		)
		.bind(work_id)
		.fetch_all(self.database.pool())
		.await
		.map_err(Into::into)
	}

	/// Resolved research line per work: override id wins, else top-topic → subfield line.
	pub async fn resolve_research_lines_for_works(
		&self,
		work_ids: &[WorkId],
		override_line_ids: &[(WorkId, Option<Uuid>)],
	) -> AppResult<std::collections::HashMap<WorkId, ResearchLineView>> {
		use std::collections::HashMap;

		if work_ids.is_empty() {
			return Ok(HashMap::new());
		}

		let mut result = HashMap::new();
		let override_map: HashMap<WorkId, Option<Uuid>> =
			override_line_ids.iter().cloned().collect();

		let override_ids: Vec<Uuid> = override_map.values().copied().flatten().collect();
		if !override_ids.is_empty() {
			let lines = sqlx::query_as::<_, ResearchLineView>(
				"SELECT id, name, slug FROM research_lines WHERE id = ANY($1)",
			)
			.bind(&override_ids)
			.fetch_all(self.database.pool())
			.await?;

			let by_id: HashMap<Uuid, ResearchLineView> =
				lines.into_iter().map(|l| (l.id, l)).collect();

			for (work_id, line_id) in &override_map {
				if let Some(id) = line_id
					&& let Some(line) = by_id.get(id)
				{
					result.insert(work_id.clone(), ResearchLineView {
						id: line.id,
						name: line.name.clone(),
						slug: line.slug.clone(),
					});
				}
			}
		}

		let missing: Vec<WorkId> = work_ids
			.iter()
			.filter(|id| !result.contains_key(id))
			.cloned()
			.collect();

		if missing.is_empty() {
			return Ok(result);
		}

		let rows = sqlx::query_as::<_, (WorkId, Uuid, String, String)>(
			"SELECT DISTINCT ON (wt.work_id)
				wt.work_id, rl.id, rl.name, rl.slug
			FROM work_topic_scores wt
			JOIN topics t ON t.id = wt.topic_id
			JOIN subfields s ON s.id = t.subfield_id
			JOIN research_lines rl ON rl.id = s.research_line_id
			WHERE wt.work_id = ANY($1)
			ORDER BY wt.work_id, wt.score DESC",
		)
		.bind(&missing)
		.fetch_all(self.database.pool())
		.await?;

		for (work_id, id, name, slug) in rows {
			result.entry(work_id).or_insert(ResearchLineView { id, name, slug });
		}

		Ok(result)
	}

	pub async fn list_keywords_by_work(
		&self,
		work_id: &WorkId,
	) -> AppResult<Vec<ResearchKeywordView>> {
		sqlx::query_as::<_, ResearchKeywordView>(
			"SELECT wk.keyword_id, k.name, wk.score
			FROM work_keyword_scores wk
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
