use crate::academic::{AcademicId, Degree, DegreeId};
use crate::shared::{AppError, AppResult, Database, Tx};

use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct DegreesRepository {
	database: Arc<Database>,
}

impl DegreesRepository {
	pub async fn list(&self, academic_id: &AcademicId) -> AppResult<Vec<Degree>> {
		Degree::all()
			.filter(Degree::fields().academic_id().eq(academic_id))
			.order_by(Degree::fields().obtained_at().desc())
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)
	}

	pub async fn find_by_id(&self, id: &DegreeId) -> AppResult<Option<Degree>> {
		Degree::filter_by_id(id)
			.first()
			.exec(&mut self.database.pool())
			.await
			.map_err(AppError::from)
	}

	pub async fn save(&self, degree: &Degree) -> AppResult<()> {
		Degree::upsert_by_id(degree.id)
			.academic_id(degree.academic_id)
			.name(&degree.name)
			.university(&degree.university)
			.obtained_at(degree.obtained_at)
			.kind(&degree.kind)
			.country_code(&degree.country_code)
			.exec(&mut self.database.pool())
			.await?;

		Ok(())
	}

	pub async fn save_tx(&self, tx: &mut Tx<'_>, degree: &Degree) -> AppResult<()> {
		Degree::upsert_by_id(degree.id)
			.academic_id(degree.academic_id)
			.name(&degree.name)
			.university(&degree.university)
			.obtained_at(degree.obtained_at)
			.kind(&degree.kind)
			.country_code(&degree.country_code)
			.exec(tx)
			.await?;

		Ok(())
	}
}
