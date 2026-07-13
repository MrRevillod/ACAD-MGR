use crate::reader::IssnRecord;
use sqlx::PgPool;

const CHUNK_SIZE: usize = 500;

pub async fn seed_records(
	pool: &PgPool,
	records: &[IssnRecord],
	kind: &str,
) -> Result<u64, Box<dyn std::error::Error>> {
	let mut total_affected = 0u64;

	for chunk in records.chunks(CHUNK_SIZE) {
		let mut tx = pool.begin().await?;
		for r in chunk {
			total_affected += upsert_record(&mut tx, r, kind).await?;
		}
		tx.commit().await?;
	}

	Ok(total_affected)
}

async fn upsert_record(
	tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
	r: &IssnRecord,
	kind: &str,
) -> Result<u64, Box<dyn std::error::Error>> {
	let existing = sqlx::query_as::<_, (i32, Option<String>, Option<String>)>(
		r#"SELECT id, issn, eissn FROM journal_issn
		   WHERE ($1::text IS NOT NULL AND issn = $1)
		      OR ($2::text IS NOT NULL AND eissn = $2)
		   ORDER BY id"#,
	)
	.bind(&r.issn)
	.bind(&r.eissn)
	.fetch_all(&mut **tx)
	.await?;

	if existing.is_empty() {
		let result = sqlx::query(
			"INSERT INTO journal_issn (issn, eissn, kinds) VALUES ($1, $2, ARRAY[$3::journal_kind])",
		)
		.bind(&r.issn)
		.bind(&r.eissn)
		.bind(kind)
		.execute(&mut **tx)
		.await?;
		return Ok(result.rows_affected());
	}

	let target_id = if existing.len() == 1 {
		existing[0].0
	} else {
		let keep_id = existing[0].0;
		let mut fill_issn = existing[0].1.clone();
		let mut fill_eissn = existing[0].2.clone();

		for (other_id, db_issn, db_eissn) in &existing[1..] {
			if fill_issn.is_none() {
				fill_issn = db_issn.clone();
			}
			if fill_eissn.is_none() {
				fill_eissn = db_eissn.clone();
			}

			let row_kinds: Option<Vec<String>> = sqlx::query_scalar(
				"SELECT kinds::text[] FROM journal_issn WHERE id = $1",
			)
			.bind(other_id)
			.fetch_optional(&mut **tx)
			.await?
			.flatten();

			sqlx::query("DELETE FROM journal_issn WHERE id = $1")
				.bind(other_id)
				.execute(&mut **tx)
				.await?;

			if let Some(kinds) = row_kinds {
				sqlx::query(
					r#"UPDATE journal_issn SET
					     kinds = ARRAY(SELECT DISTINCT unnest(journal_issn.kinds || $2::journal_kind[]))
					   WHERE id = $1"#,
				)
				.bind(keep_id)
				.bind(&kinds)
				.execute(&mut **tx)
				.await?;
			}
		}

		sqlx::query(
			r#"UPDATE journal_issn SET
			     issn = COALESCE(issn, $2),
			     eissn = COALESCE(eissn, $3)
			   WHERE id = $1"#,
		)
		.bind(keep_id)
		.bind(fill_issn.as_deref())
		.bind(fill_eissn.as_deref())
		.execute(&mut **tx)
		.await?;

		keep_id
	};

	let result = sqlx::query(
		r#"UPDATE journal_issn SET
		     issn = COALESCE(issn, $2),
		     eissn = COALESCE(eissn, $3),
		     kinds = ARRAY(SELECT DISTINCT unnest(kinds || ARRAY[$4::journal_kind]))
		   WHERE id = $1"#,
	)
	.bind(target_id)
	.bind(r.issn.as_deref())
	.bind(r.eissn.as_deref())
	.bind(kind)
	.execute(&mut **tx)
	.await?;
	Ok(result.rows_affected())
}
