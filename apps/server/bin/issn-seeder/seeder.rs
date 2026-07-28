use crate::reader::IssnRecord;

const CHUNK_SIZE: usize = 500;

pub async fn seed_records(
	db: &mut toasty::Db,
	records: &[IssnRecord],
	kind: &str,
) -> Result<u64, Box<dyn std::error::Error>> {
	let mut total = 0u64;

	for chunk in records.chunks(CHUNK_SIZE) {
		let mut tx = db.transaction().await?;
		for r in chunk {
			let issn = r.issn.as_deref().or(r.eissn.as_deref());

			if let Some(v) = issn {
				toasty::sql::statement(
					"INSERT INTO journal_issn (issn, kind) VALUES ($1, $2::journal_kind)
					 ON CONFLICT (issn) DO UPDATE SET kind = $2::journal_kind",
				)
				.bind(v)
				.bind(kind)
				.exec(&mut tx)
				.await?;

				total += 1;
			}
		}

		tx.commit().await?;
	}

	Ok(total)
}
