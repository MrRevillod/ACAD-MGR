use crate::research::sources::SourceId;
use crate::research::works::dtos::GetWorksQuery;
use crate::research::works::entity::{Work, WorkId, WorkType};
use crate::shared::{AppResult, Database};

use chrono::NaiveDate;
use sqlx::{QueryBuilder, Row};
use std::str::FromStr;
use std::sync::Arc;
use sword::prelude::*;
use uuid::Uuid;

pub struct NewWork {
	pub openalex_id: String,
	pub title: String,
	pub abstract_text: Option<String>,
	pub doi: Option<String>,
	pub publication_date: Option<NaiveDate>,
	pub publication_year: Option<i16>,
	pub ty: WorkType,
	pub lang: String,
	pub is_accepted: bool,
	pub is_published: bool,
	pub primary_source_id: Option<SourceId>,
}

#[injectable]
pub struct WorksRepository {
	database: Arc<Database>,
}

impl WorksRepository {
	pub async fn find_by_id(&self, id: &WorkId) -> AppResult<Option<Work>> {
		sqlx::query_as::<_, Work>(
            "SELECT id, openalex_id, title, abstract, doi, publication_date, publication_year, ty, lang, is_accepted, is_published, primary_source_id FROM works WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(self.database.pool())
        .await
        .map_err(Into::into)
	}

	pub async fn list(&self, query: &GetWorksQuery) -> AppResult<Vec<Work>> {
		let mut qb = QueryBuilder::new(
			"SELECT DISTINCT w.id, w.openalex_id, w.title, w.abstract, w.doi, w.publication_date, w.publication_year, w.ty, w.lang, w.is_accepted, w.is_published, w.primary_source_id FROM works w LEFT JOIN work_authorships wa ON w.id = wa.work_id LEFT JOIN work_topics wt ON w.id = wt.work_id LEFT JOIN work_keywords wk ON w.id = wk.work_id WHERE TRUE",
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

		if let Some(ref types) = query.r#type {
			let parsed: Vec<WorkType> = types
				.split(',')
				.filter_map(|s| WorkType::from_str(s.trim()).ok())
				.collect();
			if !parsed.is_empty() {
				qb.push(" AND w.ty = ANY(");
				qb.push_bind(parsed);
				qb.push(")");
			}
		}

		if let Some(domain_id) = query.domain_id {
			qb.push(" AND wt.topic_id IN (SELECT rt.id FROM research_topics rt JOIN research_subfields rs ON rs.id = rt.subfield_id JOIN research_fields rf ON rf.id = rs.field_id WHERE rf.domain_id = ");
			qb.push_bind(domain_id);
			qb.push(")");
		}

		if let Some(field_id) = query.field_id {
			qb.push(" AND wt.topic_id IN (SELECT rt.id FROM research_topics rt JOIN research_subfields rs ON rs.id = rt.subfield_id WHERE rs.field_id = ");
			qb.push_bind(field_id);
			qb.push(")");
		}

		if let Some(subfield_id) = query.subfield_id {
			qb.push(" AND wt.topic_id IN (SELECT id FROM research_topics WHERE subfield_id = ");
			qb.push_bind(subfield_id);
			qb.push(")");
		}

		if let Some(topic_id) = query.topic_id {
			if let Some(min_score) = query.topic_min_score {
				qb.push(
					" AND EXISTS (SELECT 1 FROM work_topics wt2 WHERE wt2.work_id = w.id AND wt2.topic_id = ",
				);
				qb.push_bind(topic_id);
				qb.push(" AND wt2.score >= ");
				qb.push_bind(min_score);
				qb.push(")");
			} else {
				qb.push(" AND wt.topic_id = ");
				qb.push_bind(topic_id);
			}
		}

		if let Some(keyword_id) = query.keyword_id {
			if let Some(min_score) = query.keyword_min_score {
				qb.push(
					" AND EXISTS (SELECT 1 FROM work_keywords wk2 WHERE wk2.work_id = w.id AND wk2.keyword_id = ",
				);
				qb.push_bind(keyword_id);
				qb.push(" AND wk2.score >= ");
				qb.push_bind(min_score);
				qb.push(")");
			} else {
				qb.push(" AND wk.keyword_id = ");
				qb.push_bind(keyword_id);
			}
		}

		if let Some(is_accepted) = query.is_accepted {
			qb.push(" AND w.is_accepted = ");
			qb.push_bind(is_accepted);
		}

		if let Some(is_published) = query.is_published {
			qb.push(" AND w.is_published = ");
			qb.push_bind(is_published);
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
            "INSERT INTO work_keywords (work_id, keyword_id, score) VALUES ($1, $2, $3) ON CONFLICT (work_id, keyword_id) DO NOTHING",
        )
        .bind(work_id)
        .bind(keyword_id)
        .bind(score)
        .execute(self.database.pool())
        .await?;
		Ok(())
	}

	pub async fn find_academic_name_by_orcid(&self, orcid: &str) -> AppResult<Option<String>> {
		sqlx::query_scalar::<_, String>(
            "SELECT names || ' ' || paternal_surname || ' ' || maternal_surname FROM academics WHERE orcid = $1",
        )
        .bind(orcid)
        .fetch_optional(self.database.pool())
        .await
        .map_err(Into::into)
	}

	pub async fn academic_has_orcid(&self, academic_id: Uuid) -> AppResult<Option<String>> {
		sqlx::query_scalar::<_, String>("SELECT orcid FROM academics WHERE id = $1")
			.bind(academic_id)
			.fetch_optional(self.database.pool())
			.await
			.map_err(Into::into)
	}
}
