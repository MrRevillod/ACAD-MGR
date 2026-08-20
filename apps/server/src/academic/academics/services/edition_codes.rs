use crate::academic::{AcademicId, EditCode, EditCodeId, EditCodesRepository};
use crate::shared::AppResult;

use chrono::Utc;
use std::sync::Arc;
use sword::prelude::*;
use uuid::Uuid;

const CODE_ALPHABET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
const CODE_LENGTH: usize = 8;
const TARGET_VIGENTES: usize = 10;

#[injectable]
pub struct EditCodesService {
	repository: Arc<EditCodesRepository>,
}

impl EditCodesService {
	fn generate_code() -> String {
		let uuid = Uuid::new_v4();
		let mut output = String::with_capacity(CODE_LENGTH);

		for i in 0..CODE_LENGTH {
			let byte = uuid.as_bytes()[i];
			let idx = (byte as usize) % CODE_ALPHABET.len();
			output.push(CODE_ALPHABET[idx] as char);
		}

		output
	}

	pub async fn ensure_vigentes(&self, academic_id: &AcademicId) -> AppResult<Vec<EditCode>> {
		let current = self.repository.count_vigentes(academic_id).await?;
		let mut missing = TARGET_VIGENTES.saturating_sub(current as usize);

		let mut new_codes = Vec::with_capacity(missing);

		while missing > 0 {
			let code = EditCode::builder()
				.id(EditCodeId::new())
				.academic_id(*academic_id)
				.code(Self::generate_code())
				.created_at(Utc::now())
				.build();

			new_codes.push(code);
			missing -= 1;
		}

		if !new_codes.is_empty() {
			self.repository.insert_many(&new_codes).await?;
		}

		let mut all = self.repository.list_vigentes(academic_id).await?;
		all.extend(new_codes);
		Ok(all)
	}

	pub async fn consume(&self, code: &str) -> AppResult<Option<AcademicId>> {
		let Some(edit_code) = self.repository.find_by_code(code).await? else {
			return Ok(None);
		};

		self.repository.mark_used(&edit_code.id).await?;
		Ok(Some(edit_code.academic_id))
	}
}
