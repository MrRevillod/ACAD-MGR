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
            w.is_published, w.primary_source_id, ji.kind::text AS journal_kind
            FROM works w LEFT JOIN sources src ON w.primary_source_id = src.id
            LEFT JOIN LATERAL (SELECT kind FROM journal_issn WHERE issn = src.issn_l
            OR eissn = src.issn_l OR issn = ANY(src.issn) OR eissn = ANY(src.issn) LIMIT 1) ji ON TRUE
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
				w.is_published, w.primary_source_id, ji.kind::text AS journal_kind
			FROM works w
			LEFT JOIN work_authorships wa ON w.id = wa.work_id
			LEFT JOIN sources src ON w.primary_source_id = src.id
			LEFT JOIN LATERAL (
			SELECT kind FROM journal_issn WHERE issn = src.issn_l OR eissn = src.issn_l OR issn = ANY(src.issn) OR eissn = ANY(src.issn) LIMIT 1) ji ON TRUE WHERE TRUE",
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

		qb.push(
			" ORDER BY w.publication_year DESC NULLS LAST, w.publication_date DESC NULLS LAST, w.id LIMIT ",
		);

		qb.push_bind(query.size.unwrap_or(100) as i64);

		qb.build_query_as()
			.fetch_all(self.database.pool())
			.await
			.map_err(Into::into)
	}

	pub async fn insert_work(&self, work: &NewWork) -> AppResult<Option<WorkId>> {
		let row = sqlx::query(
            "INSERT INTO works (openalex_id, title, abstract, doi, publication_date, publication_year, ty, lang, is_accepted, is_published, primary_source_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) ON CONFLICT (openalex_id) DO NOTHING RETURNING id",
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
        .fetch_optional(self.database.pool())
        .await?;
		Ok(row
			.map(|r| SourceId::from_uuid(r.get("id")))
			.map(|sid| WorkId::from_uuid(*sid)))
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
