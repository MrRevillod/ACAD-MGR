use crate::research::*;
use crate::shared::{AppError, AppResult, Database};

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct WorkClassificationRepository {
	database: Arc<Database>,
}

impl WorkClassificationRepository {
	pub async fn list_domains(&self, f: ClassificationFilter) -> AppResult<Vec<ResearchDomain>> {
		let mut query = ResearchDomain::all();

		if let Some(s) = f.search {
			let pattern = format!("%{}%", s.trim());
			query = query.filter(ResearchDomain::fields().name().ilike(pattern))
		}

		query = query.order_by(ResearchDomain::fields().name().asc());

		query
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)
	}

	pub async fn list_fields(&self, f: ClassificationFilter) -> AppResult<Vec<ResearchField>> {
		let mut fields = ResearchField::all();

		if let Some(domain_id) = f.domain_id {
			fields = fields.filter(ResearchField::fields().domain_id().eq(domain_id));
		}

		if let Some(s) = f.search {
			let pattern = format!("%{}%", s.trim());
			fields = fields.filter(ResearchField::fields().name().ilike(pattern))
		}

		fields
			.order_by(ResearchField::fields().name().asc())
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)
	}

	pub async fn list_subfields(
		&self,
		f: ClassificationFilter,
	) -> AppResult<Vec<ResearchSubfield>> {
		let mut subfields = ResearchSubfield::all();

		if let Some(field_id) = f.field_id {
			subfields = subfields.filter(ResearchSubfield::fields().field_id().eq(field_id));
		}

		if let Some(s) = f.search {
			let pattern = format!("%{}%", s.trim());
			subfields = subfields.filter(ResearchSubfield::fields().name().ilike(pattern))
		}

		subfields
			.order_by(ResearchSubfield::fields().name().asc())
			.limit(50)
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)
	}

	pub async fn list_topics(&self, f: ClassificationFilter) -> AppResult<Vec<ResearchTopic>> {
		let mut topics = ResearchTopic::all();

		if let Some(subfield_id) = f.subfield_id {
			topics = topics.filter(ResearchTopic::fields().subfield_id().eq(subfield_id));
		}

		if let Some(s) = f.search {
			let pattern = format!("%{}%", s.trim());
			topics = topics.filter(ResearchTopic::fields().name().ilike(pattern));
		}

		topics
			.order_by(ResearchTopic::fields().name().asc())
			.limit(50)
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)
	}

	pub async fn list_keywords(&self, f: ClassificationFilter) -> AppResult<Vec<ResearchKeyword>> {
		let mut query = ResearchKeyword::all();

		if let Some(search) = f.search {
			let pattern = format!("%{}%", search.trim());
			query = query.filter(ResearchKeyword::fields().name().ilike(pattern));
		}

		query
			.order_by(ResearchKeyword::fields().name().asc())
			.limit(50)
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)
	}

	pub async fn list_research_lines(&self) -> AppResult<Vec<ResearchLine>> {
		ResearchLine::all()
			.include(ResearchLine::fields().subfields())
			.order_by(ResearchLine::fields().name().asc())
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)
	}

	pub async fn find_subfield_by_id(
		&self,
		id: &ResearchSubfieldId,
	) -> AppResult<Option<ResearchSubfield>> {
		ResearchSubfield::filter_by_id(id)
			.first()
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)
	}

	pub async fn find_research_line_by_id(
		&self,
		id: &ResearchLineId,
	) -> AppResult<Option<ResearchLine>> {
		ResearchLine::filter_by_id(id)
			.first()
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)
	}

	pub async fn find_topic_by_openalex_id(
		&self,
		openalex_id: &str,
	) -> AppResult<Option<ResearchTopic>> {
		ResearchTopic::filter(ResearchTopic::fields().openalex_id().eq(openalex_id))
			.first()
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)
	}

	pub async fn save_keyword(&self, keyword: &ResearchKeyword) -> AppResult<()> {
		ResearchKeyword::upsert_by_openalex_id(&keyword.openalex_id)
			.id(&keyword.id)
			.name(&keyword.name)
			.exec(&mut self.database.pool())
			.await?;

		Ok(())
	}

	pub async fn save_subfield(&self, subfield: &ResearchSubfield) -> AppResult<()> {
		ResearchSubfield::upsert_by_openalex_id(&subfield.openalex_id)
			.id(&subfield.id)
			.name(&subfield.name)
			.field_id(&subfield.field_id)
			.research_line_id(subfield.research_line_id)
			.exec(&mut self.database.pool())
			.await?;

		Ok(())
	}

	pub async fn unknown_keyword_id(&self) -> AppResult<Option<ResearchKeyword>> {
		ResearchKeyword::filter(ResearchKeyword::fields().openalex_id().eq("unknown"))
			.first()
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)
	}

	pub async fn unknown_topic_id(&self) -> AppResult<Option<ResearchTopic>> {
		ResearchTopic::filter(ResearchTopic::fields().openalex_id().eq("unknown"))
			.first()
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)
	}
}
