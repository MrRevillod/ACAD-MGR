use crate::research::*;
use crate::shared::{AppError, AppResult, Database};

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct WorkClassificationRepository {
	database: Arc<Database>,
}

impl WorkClassificationRepository {
	pub async fn list_domains(&self, f: ClassificationFilter) -> AppResult<Vec<Domain>> {
		let mut query = Domain::all();

		if let Some(s) = f.search {
			let pattern = format!("%{}%", s.trim());
			query = query.filter(Domain::fields().name().ilike(pattern))
		}

		query = query.order_by(Domain::fields().name().asc());

		query
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)
	}

	pub async fn list_fields(&self, f: ClassificationFilter) -> AppResult<Vec<Field>> {
		let mut fields = Field::all();

		if let Some(domain_id) = f.domain_id {
			fields = fields.filter(Field::fields().domain_id().eq(domain_id));
		}

		if let Some(s) = f.search {
			let pattern = format!("%{}%", s.trim());
			fields = fields.filter(Field::fields().name().ilike(pattern))
		}

		fields
			.order_by(Field::fields().name().asc())
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)
	}

	pub async fn list_subfields(&self, f: ClassificationFilter) -> AppResult<Vec<Subfield>> {
		let mut subfields = Subfield::all();

		if let Some(field_id) = f.field_id {
			subfields = subfields.filter(Subfield::fields().field_id().eq(field_id));
		}

		if let Some(s) = f.search {
			subfields = subfields.filter(Subfield::fields().name().ilike(format!("%{}%", s.trim())))
		}

		subfields
			.order_by(Subfield::fields().name().asc())
			.limit(50)
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)
	}

	pub async fn list_topics(&self, f: ClassificationFilter) -> AppResult<Vec<Topic>> {
		let mut topics = Topic::all();

		if let Some(subfield_id) = f.subfield_id {
			topics = topics.filter(Topic::fields().subfield_id().eq(subfield_id));
		}

		if let Some(s) = f.search {
			topics = topics.filter(Topic::fields().name().ilike(format!("%{}%", s.trim())));
		}

		topics
			.order_by(Topic::fields().name().asc())
			.limit(50)
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)
	}

	pub async fn list_keywords(&self, f: ClassificationFilter) -> AppResult<Vec<Keyword>> {
		let mut query = Keyword::all();

		if let Some(search) = f.search {
			query = query.filter(
				Keyword::fields()
					.name()
					.ilike(format!("%{}%", search.trim())),
			);
		}

		query
			.order_by(Keyword::fields().name().asc())
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

	pub async fn find_subfield_by_id(&self, id: &SubfieldId) -> AppResult<Option<Subfield>> {
		Subfield::filter_by_id(id)
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

	pub async fn find_topic_by_openalex_id(&self, openalex_id: &str) -> AppResult<Option<Topic>> {
		Topic::filter(Topic::fields().openalex_id().eq(openalex_id))
			.first()
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)
	}

	pub async fn save_keyword(&self, keyword: &Keyword) -> AppResult<()> {
		Keyword::upsert_by_openalex_id(&keyword.openalex_id)
			.id(&keyword.id)
			.name(&keyword.name)
			.exec(&mut self.database.pool())
			.await?;

		Ok(())
	}

	pub async fn save_subfield(&self, subfield: &Subfield) -> AppResult<()> {
		Subfield::upsert_by_openalex_id(&subfield.openalex_id)
			.id(&subfield.id)
			.name(&subfield.name)
			.field_id(&subfield.field_id)
			.research_line_id(subfield.research_line_id)
			.exec(&mut self.database.pool())
			.await?;

		Ok(())
	}

	pub async fn unknown_keyword_id(&self) -> AppResult<Option<Keyword>> {
		Keyword::filter(Keyword::fields().openalex_id().eq("unknown"))
			.first()
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)
	}

	pub async fn unknown_topic_id(&self) -> AppResult<Option<Topic>> {
		Topic::filter(Topic::fields().openalex_id().eq("unknown"))
			.first()
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)
	}
}
