use crate::research::*;
use crate::shared::{AppResult, Database};

use sqlx::{QueryBuilder, Row};
use std::sync::Arc;
use sword::prelude::*;
use uuid::Uuid;

#[injectable]
pub struct WorksRepository {
	database: Arc<Database>,
}

impl WorksRepository {
	pub async fn find_by_id(&self, id: &WorkId) -> AppResult<Option<Work>> {
		sqlx::query_as::<_, Work>(
            "SELECT w.id, w.openalex_id, w.title, w.abstract, w.doi,
            w.publication_date, w.publication_year, w.ty, w.lang, w.is_accepted,
            w.is_published, w.primary_source_id, w.overrides, ji.kind::text AS journal_kind,
            rl.id AS research_line_id, rl.name AS research_line_name, rl.slug AS research_line_slug
            FROM works w LEFT JOIN sources src ON w.primary_source_id = src.id
            LEFT JOIN LATERAL (SELECT kind FROM journal_issn WHERE issn = src.issn_l
            OR eissn = src.issn_l OR issn = ANY(src.issn) OR eissn = ANY(src.issn) LIMIT 1) ji ON TRUE
            LEFT JOIN LATERAL (
                SELECT r.id, r.name, r.slug
                FROM work_topics wt
                JOIN research_topics rt ON rt.id = wt.topic_id
                JOIN research_subfields rs ON rs.id = rt.subfield_id
                LEFT JOIN research_line_mappings rlm ON rlm.subfield_openalex_id = rs.openalex_id
                LEFT JOIN work_research_line_overrides o ON o.work_id = w.id
                JOIN research_lines r ON r.id = COALESCE(o.research_line_id, rlm.research_line_id)
                WHERE wt.work_id = w.id
                ORDER BY wt.score DESC
                LIMIT 1
            ) rl ON TRUE
            WHERE w.id = $1",
        )
        .bind(id)
        .fetch_optional(self.database.pool())
        .await
        .map_err(Into::into)
	}

	pub async fn list(&self, query: &GetWorksQuery) -> AppResult<Vec<Work>> {
		let mut qb = QueryBuilder::new(
			"SELECT DISTINCT w.id, w.openalex_id, w.title, w.abstract,
				w.doi, w.publication_date, w.publication_year, w.ty, w.lang, w.is_accepted,
				w.is_published, w.primary_source_id, w.overrides, ji.kind::text AS journal_kind,
				rl.id AS research_line_id, rl.name AS research_line_name, rl.slug AS research_line_slug
			FROM works w
			LEFT JOIN work_authorships wa ON w.id = wa.work_id
			LEFT JOIN sources src ON w.primary_source_id = src.id
			LEFT JOIN LATERAL (
			SELECT kind FROM journal_issn WHERE issn = src.issn_l OR eissn = src.issn_l OR issn = ANY(src.issn) OR eissn = ANY(src.issn) LIMIT 1) ji ON TRUE
			LEFT JOIN LATERAL (
				SELECT r.id, r.name, r.slug
				FROM work_topics wt
				JOIN research_topics rt ON rt.id = wt.topic_id
				JOIN research_subfields rs ON rs.id = rt.subfield_id
				LEFT JOIN research_line_mappings rlm ON rlm.subfield_openalex_id = rs.openalex_id
				LEFT JOIN work_research_line_overrides o ON o.work_id = w.id
				JOIN research_lines r ON r.id = COALESCE(o.research_line_id, rlm.research_line_id)
				WHERE wt.work_id = w.id
				ORDER BY wt.score DESC
				LIMIT 1
			) rl ON TRUE
			WHERE TRUE",
		);

		if let Some(academic_id) = query.academic_id {
			qb.push(" AND wa.work_id IN (SELECT wa2.work_id FROM work_authorships wa2 JOIN academics a ON a.orcid = wa2.orcid WHERE wa2.is_external = false AND a.orcid != 'https://orcid.org/0000-0000-0000-0000' AND a.id = ");
			qb.push_bind(academic_id);
			qb.push(")");
		}

		if let Some(department_id) = query.department_id {
			qb.push(" AND wa.work_id IN (SELECT wa2.work_id FROM work_authorships wa2 JOIN academics a ON a.orcid = wa2.orcid WHERE wa2.is_external = false AND a.orcid != 'https://orcid.org/0000-0000-0000-0000' AND a.department_id = ");
			qb.push_bind(department_id);
			qb.push(")");
		}

		if let Some(career_id) = query.career_id {
			qb.push(" AND wa.work_id IN (SELECT wa2.work_id FROM work_authorships wa2 JOIN academics a ON a.orcid = wa2.orcid WHERE wa2.is_external = false AND a.orcid != 'https://orcid.org/0000-0000-0000-0000' AND a.career_id = ");
			qb.push_bind(career_id);
			qb.push(")");
		}

		if let Some(ref search) = query.search {
			qb.push(" AND w.title ILIKE ");
			qb.push_bind(format!("%{}%", search));
		}

		if let Some(year_from) = query.year_from {
			qb.push(" AND w.publication_year >= ");
			qb.push_bind(year_from);
		}

		if let Some(year_to) = query.year_to {
			qb.push(" AND w.publication_year <= ");
			qb.push_bind(year_to);
		}

		if let Some(is_accepted) = query.is_accepted {
			qb.push(" AND w.is_accepted = ");
			qb.push_bind(is_accepted);
		}

		if let Some(is_published) = query.is_published {
			qb.push(" AND w.is_published = ");
			qb.push_bind(is_published);
		}

		if let Some(ref journal_kind) = query.journal_kind {
			qb.push(" AND (ji.kind = ");
			qb.push_bind(journal_kind);
			qb.push("::journal_kind)");
		}

		if let Some(research_line_id) = query.research_line_id {
			qb.push(" AND rl.id = ");
			qb.push_bind(research_line_id);
		}

		qb.push(
			" ORDER BY w.publication_year DESC NULLS LAST, w.publication_date DESC NULLS LAST, w.id LIMIT ",
		);

		qb.push_bind(query.size.unwrap_or(100) as i64);

		qb.build_query_as()
			.fetch_all(self.database.pool())
			.await
			.map_err(Into::into)
	}

	pub async fn upsert_work(&self, work: &NewWork) -> AppResult<(WorkId, bool)> {
		let row = sqlx::query(
            "INSERT INTO works (openalex_id, title, abstract, doi, publication_date, publication_year, ty, lang, is_accepted, is_published, primary_source_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) ON CONFLICT (openalex_id) DO UPDATE SET title = EXCLUDED.title, abstract = EXCLUDED.abstract, doi = EXCLUDED.doi, publication_date = EXCLUDED.publication_date, publication_year = EXCLUDED.publication_year, ty = EXCLUDED.ty, lang = EXCLUDED.lang, is_accepted = EXCLUDED.is_accepted, is_published = EXCLUDED.is_published, primary_source_id = EXCLUDED.primary_source_id RETURNING id, (xmax = 0) AS was_inserted",
        )
        .bind(&work.openalex_id)
        .bind(&work.title)
        .bind(&work.abstract_text)
        .bind(&work.doi)
        .bind(work.publication_date)
        .bind(work.publication_year)
        .bind(work.ty)
        .bind(&work.lang)
        .bind(work.is_accepted)
        .bind(work.is_published)
        .bind(work.primary_source_id)
        .fetch_one(self.database.pool())
        .await?;

		let id = WorkId::from_uuid(row.get::<Uuid, _>("id"));
		let was_inserted: bool = row.get("was_inserted");
		Ok((id, was_inserted))
	}

	pub async fn apply_overrides_sync(&self, work_id: &WorkId) -> AppResult<()> {
		sqlx::query(
			"UPDATE works SET
				title = COALESCE(overrides->>'title', title),
				doi = COALESCE(overrides->>'doi', doi),
				is_accepted = COALESCE((overrides->>'is_accepted')::boolean, is_accepted),
				is_published = COALESCE((overrides->>'is_published')::boolean, is_published),
				publication_year = COALESCE((overrides->>'publication_year')::smallint, publication_year)
			WHERE id = $1 AND overrides != '{}'::jsonb",
		)
		.bind(work_id)
		.execute(self.database.pool())
		.await?;

		Ok(())
	}

	pub async fn update_overrides(
		&self,
		work_id: &WorkId,
		overrides: &serde_json::Value,
	) -> AppResult<()> {
		sqlx::query("UPDATE works SET overrides = $1 WHERE id = $2")
			.bind(overrides)
			.bind(work_id)
			.execute(self.database.pool())
			.await?;

		Ok(())
	}

	pub async fn clear_overrides(&self, work_id: &WorkId) -> AppResult<()> {
		sqlx::query("UPDATE works SET overrides = '{}'::jsonb WHERE id = $1")
			.bind(work_id)
			.execute(self.database.pool())
			.await?;

		Ok(())
	}

	pub async fn link_topic(&self, work_id: &WorkId, topic_id: Uuid, score: f64) -> AppResult<()> {
		sqlx::query(
            "INSERT INTO work_topics (work_id, topic_id, score) VALUES ($1, $2, $3) ON CONFLICT (work_id, topic_id) DO NOTHING",
        )
        .bind(work_id)
        .bind(topic_id)
        .bind(score)
        .execute(self.database.pool())
        .await?;

		Ok(())
	}

	pub async fn link_keyword(
		&self,
		work_id: &WorkId,
		keyword_id: Uuid,
		score: f64,
	) -> AppResult<()> {
		sqlx::query(
			"INSERT INTO work_keywords (work_id, keyword_id, score)
            VALUES ($1, $2, $3) ON CONFLICT (work_id, keyword_id) DO NOTHING",
		)
		.bind(work_id)
		.bind(keyword_id)
		.bind(score)
		.execute(self.database.pool())
		.await?;

		Ok(())
	}
}
