use crate::research::*;
use crate::shared::{AppError, AppResult, Database};

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct WorksRepository {
	database: Arc<Database>,
}

impl WorksRepository {
	pub async fn find_by_id(&self, id: &WorkId) -> AppResult<Option<Work>> {
		Work::filter_by_id(id)
			.include(Work::fields().authorships())
			.include(Work::fields().topic_scores())
			.include(Work::fields().topics())
			.include(Work::fields().keyword_scores())
			.include(Work::fields().keywords())
			.first()
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)
	}

	pub async fn list(&self, query: &GetWorksQuery) -> AppResult<Vec<Work>> {
		let mut works = Work::all()
			.include(Work::fields().topic_scores())
			.include(Work::fields().keyword_scores())
			.order_by(Work::fields().publication_year().desc());

		if let Some(academic_id) = query.academic_id {
			works = works.filter(
				Work::fields().authorships().any(
					Authorship::fields()
						.academic_id()
						.eq(academic_id)
						.and(Authorship::fields().is_external().eq(false)),
				),
			);
		}

		if let Some(ref search) = query.search {
			works = works.filter(Work::fields().title().like(format!("%{}%", search)));
		}

		if let Some(year_from) = query.year_from {
			works = works.filter(Work::fields().publication_year().ge(year_from));
		}

		if let Some(year_to) = query.year_to {
			works = works.filter(Work::fields().publication_year().le(year_to));
		}

		if let Some(is_accepted) = query.is_accepted {
			works = works.filter(Work::fields().is_accepted().eq(is_accepted));
		}

		if let Some(is_published) = query.is_published {
			works = works.filter(Work::fields().is_published().eq(is_published));
		}

		works = works
			.order_by(Work::fields().publication_year().desc())
			.order_by(Work::fields().publication_date().desc());

		if let Some(size) = query.size {
			works = works.limit(size as usize);
		}

		works
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)
	}

	pub async fn save(&self, work: &Work) -> AppResult<()> {
		Work::upsert_by_id(work.id)
			.title(&work.title)
			.abstract_text(&work.abstract_text)
			.doi(&work.doi)
			.publication_date(work.publication_date)
			.publication_year(work.publication_year)
			.ty(work.ty)
			.lang(&work.lang)
			.is_accepted(work.is_accepted)
			.is_published(work.is_published)
			.source_id(work.source_id)
			.overrides(&work.overrides)
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from);

		Ok(())
	}

	pub async fn link_topic(&self, work_topic_score: &WorkTopicScore) -> AppResult<()> {
		WorkTopicScore::create()
			.work_id(work_topic_score.work_id)
			.topic_id(work_topic_score.topic_id)
			.score(work_topic_score.score)
			.exec(&mut self.database.pool())
			.await?;

		Ok(())
	}

	pub async fn link_keyword(&self, work_keyword_score: &WorkKeyWordScore) -> AppResult<()> {
		WorkKeyWordScore::create()
			.work_id(work_keyword_score.work_id)
			.keyword_id(work_keyword_score.keyword_id)
			.score(work_keyword_score.score)
			.exec(&mut self.database.pool())
			.await?;

		Ok(())
	}
}
