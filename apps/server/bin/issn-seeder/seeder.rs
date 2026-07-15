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
	if kind == "wos" {
		if let Some(ref issn) = r.issn {
			return upsert_issn_wos(tx, issn, r.eissn.as_deref()).await;
		}
		if let Some(ref eissn) = r.eissn {
			return upsert_eissn_wos(tx, eissn).await;
		}
	} else {
		if let Some(ref issn) = r.issn {
			return upsert_issn_scopus(tx, issn, r.eissn.as_deref()).await;
		}
		if let Some(ref eissn) = r.eissn {
			return upsert_eissn_scopus(tx, eissn).await;
		}
	}
	Ok(0)
}

async fn upsert_issn_wos(
	tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
	issn: &str,
	eissn: Option<&str>,
) -> Result<u64, Box<dyn std::error::Error>> {
	sqlx::query("SAVEPOINT wos_issn_sp")
		.execute(&mut **tx)
		.await?;

	let result = sqlx::query(
		r#"INSERT INTO journal_issn (issn, eissn, kind)
		   VALUES ($1, $2, 'wos'::journal_kind)
		   ON CONFLICT (issn) DO UPDATE SET
		     eissn = COALESCE(journal_issn.eissn, EXCLUDED.eissn),
		     kind = EXCLUDED.kind"#,
	)
	.bind(issn)
	.bind(eissn)
	.execute(&mut **tx)
	.await;

	match result {
		Ok(r) => {
			sqlx::query("RELEASE SAVEPOINT wos_issn_sp")
				.execute(&mut **tx)
				.await?;
			Ok(r.rows_affected())
		}
		Err(sqlx::Error::Database(ref e)) if e.is_unique_violation() => {
			sqlx::query("ROLLBACK TO SAVEPOINT wos_issn_sp")
				.execute(&mut **tx)
				.await?;
			if let Some(eissn_val) = eissn {
				let r = sqlx::query(
					r#"UPDATE journal_issn SET
					     issn = COALESCE(issn, $1),
					     kind = 'wos'::journal_kind
					   WHERE eissn = $2"#,
				)
				.bind(issn)
				.bind(eissn_val)
				.execute(&mut **tx)
				.await?;
				Ok(r.rows_affected())
			} else {
				Ok(0)
			}
		}
		Err(e) => Err(Box::new(e)),
	}
}

async fn upsert_eissn_wos(
	tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
	eissn: &str,
) -> Result<u64, Box<dyn std::error::Error>> {
	let result = sqlx::query(
		r#"INSERT INTO journal_issn (eissn, kind)
		   VALUES ($1, 'wos'::journal_kind)
		   ON CONFLICT (eissn) DO UPDATE SET kind = EXCLUDED.kind"#,
	)
	.bind(eissn)
	.execute(&mut **tx)
	.await?;
	Ok(result.rows_affected())
}

async fn upsert_issn_scopus(
	tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
	issn: &str,
	eissn: Option<&str>,
) -> Result<u64, Box<dyn std::error::Error>> {
	if let Some(eissn_val) = eissn
		&& let Some(id) = find_id_by_eissn(tx, eissn_val).await?
	{
		return update_issin_by_id(tx, id, issn).await;
	}

	if let Some(id) = find_id_by_issn(tx, issn).await? {
		if let Some(eissn_val) = eissn {
			return update_eissn_by_id(tx, id, eissn_val).await;
		}
		return Ok(0);
	}

	let result = sqlx::query(
		"INSERT INTO journal_issn (issn, eissn, kind) VALUES ($1, $2, 'scopus'::journal_kind)",
	)
	.bind(issn)
	.bind(eissn)
	.execute(&mut **tx)
	.await?;
	Ok(result.rows_affected())
}

async fn upsert_eissn_scopus(
	tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
	eissn: &str,
) -> Result<u64, Box<dyn std::error::Error>> {
	if find_id_by_eissn(tx, eissn).await?.is_some() {
		return Ok(0);
	}

	let result = sqlx::query(
		"INSERT INTO journal_issn (eissn, kind) VALUES ($1, 'scopus'::journal_kind)",
	)
	.bind(eissn)
	.execute(&mut **tx)
	.await?;
	Ok(result.rows_affected())
}

async fn find_id_by_eissn(
	tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
	value: &str,
) -> Result<Option<i32>, Box<dyn std::error::Error>> {
	sqlx::query_scalar::<_, i32>("SELECT id FROM journal_issn WHERE eissn = $1")
		.bind(value)
		.fetch_optional(&mut **tx)
		.await
		.map_err(Into::into)
}

async fn find_id_by_issn(
	tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
	value: &str,
) -> Result<Option<i32>, Box<dyn std::error::Error>> {
	sqlx::query_scalar::<_, i32>("SELECT id FROM journal_issn WHERE issn = $1")
		.bind(value)
		.fetch_optional(&mut **tx)
		.await
		.map_err(Into::into)
}

async fn update_issin_by_id(
	tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
	id: i32,
	issn: &str,
) -> Result<u64, Box<dyn std::error::Error>> {
	let result = sqlx::query("UPDATE journal_issn SET issn = COALESCE(issn, $1) WHERE id = $2")
		.bind(issn)
		.bind(id)
		.execute(&mut **tx)
		.await?;
	Ok(result.rows_affected())
}

async fn update_eissn_by_id(
	tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
	id: i32,
	eissn: &str,
) -> Result<u64, Box<dyn std::error::Error>> {
	let result = sqlx::query("UPDATE journal_issn SET eissn = COALESCE(eissn, $1) WHERE id = $2")
		.bind(eissn)
		.bind(id)
		.execute(&mut **tx)
		.await?;
	Ok(result.rows_affected())
}
