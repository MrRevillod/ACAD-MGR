use crate::academic::*;
use crate::shared::{AppError, AppResult, Database, Tx};

use jiff::Timestamp;
use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct AcademicsRepository {
	database: Arc<Database>,
}

impl AcademicsRepository {
	pub async fn list(&self, filter: AcademicListFilter) -> AppResult<Vec<AcademicView>> {
		let mut query = Academic::all()
			.include(Academic::fields().degrees())
			.include(Academic::fields().department())
			.include(Academic::fields().career())
			.include(Academic::fields().work_position())
			.include(Academic::fields().category_option().category());

		if let Some(q) = filter.search {
			let pattern = format!("%{}%", q.trim());

			let pattern_chain = Academic::fields()
				.names()
				.ilike(&pattern)
				.or(Academic::fields().paternal_surname().ilike(&pattern))
				.or(Academic::fields().maternal_surname().ilike(&pattern))
				.or(Academic::fields().email().ilike(&pattern));

			query = query.filter(pattern_chain);
		}

		if let Some(id) = filter.department_id {
			query = query.filter(Academic::fields().department_id().eq(id));
		}

		if let Some(id) = filter.career_id {
			query = query.filter(Academic::fields().career_id().eq(id));
		}

		if let Some(id) = filter.category_id {
			query = query.filter(Academic::fields().category_option().category_id().eq(id));
		}

		if let Some(planta) = filter.planta {
			let chain = Academic::fields()
				.category_option()
				.category()
				.planta()
				.eq(planta);

			query = query.filter(chain);
		}

		if let Some(option) = filter.option {
			query = query.filter(Academic::fields().category_option().option().eq(option));
		}

		let academics = query
			.exec(&mut self.database.pool())
			.await?
			.into_iter()
			.map(AcademicView::from)
			.collect();

		Ok(academics)
	}

	pub async fn find_view_by_id(&self, id: &AcademicId) -> AppResult<Option<AcademicView>> {
		Ok(self.find_by_id(id).await?.map(AcademicView::from))
	}

	pub async fn find_by_id(&self, id: &AcademicId) -> AppResult<Option<Academic>> {
		let academic = Academic::filter_by_id(id)
			.include(Academic::fields().degrees())
			.include(Academic::fields().department())
			.include(Academic::fields().career())
			.include(Academic::fields().work_position())
			.include(Academic::fields().category_option().category())
			.first()
			.exec(&mut self.database.pool())
			.await?;

		Ok(academic)
	}

	pub async fn find_by_rut(&self, rut: &str) -> AppResult<Option<Academic>> {
		let academic = Academic::filter_by_rut(rut)
			.first()
			.exec(&mut self.database.pool())
			.await?;

		Ok(academic)
	}

	pub async fn find_by_orcid(&self, orcid: &str) -> AppResult<Option<Academic>> {
		let academic = Academic::filter_by_orcid(orcid)
			.first()
			.exec(&mut self.database.pool())
			.await?;

		Ok(academic)
	}

	pub async fn update_updated_at(&self, id: &AcademicId) -> AppResult<Timestamp> {
		let now = Timestamp::now();

		Academic::update_by_id(id)
			.updated_at(now)
			.exec(&mut self.database.pool())
			.await?;

		Ok(now)
	}

	pub async fn save(&self, academic: &Academic) -> AppResult<()> {
		Academic::upsert_by_id(academic.id)
			.rut(&academic.rut)
			.names(&academic.names)
			.paternal_surname(&academic.paternal_surname)
			.maternal_surname(&academic.maternal_surname)
			.email(&academic.email)
			.orcid(&academic.orcid)
			.sex(academic.sex)
			.birth_date(academic.birth_date)
			.joined_at(academic.joined_at)
			.work_position_id(academic.work_position_id)
			.department_id(academic.department_id)
			.career_id(academic.career_id)
			.jce(academic.jce)
			.category_option_id(academic.category_option_id)
			.annual_discount_hours(academic.annual_discount_hours)
			.nationality_code(&academic.nationality_code)
			.city(&academic.city)
			.updated_at(academic.updated_at)
			.exec(&mut self.database.pool())
			.await?;

		Ok(())
	}

	pub async fn save_tx(&self, tx: &mut Tx<'_>, academic: &Academic) -> AppResult<()> {
		Academic::upsert_by_id(academic.id)
			.rut(&academic.rut)
			.names(&academic.names)
			.paternal_surname(&academic.paternal_surname)
			.maternal_surname(&academic.maternal_surname)
			.email(&academic.email)
			.orcid(&academic.orcid)
			.sex(academic.sex)
			.birth_date(academic.birth_date)
			.joined_at(academic.joined_at)
			.work_position_id(academic.work_position_id)
			.department_id(academic.department_id)
			.career_id(academic.career_id)
			.jce(academic.jce)
			.category_option_id(academic.category_option_id)
			.annual_discount_hours(academic.annual_discount_hours)
			.nationality_code(&academic.nationality_code)
			.city(&academic.city)
			.updated_at(academic.updated_at)
			.exec(tx)
			.await?;

		Ok(())
	}

	pub async fn list_orcids(&self) -> AppResult<Vec<(AcademicId, Option<String>)>> {
		Academic::all()
			.filter(Academic::fields().orcid().is_some())
			.select((Academic::fields().id(), Academic::fields().orcid()))
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)
	}
}
