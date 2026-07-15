use std::error::Error;
use std::path::Path;

mod reader;
mod seeder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let database_url = std::env::var("LOCAL_POSTGRES_DATABASE_URL")
		.or_else(|_| std::env::var("POSTGRES_DATABASE_URL"))?;

	let pool = sqlx::postgres::PgPoolOptions::new()
		.max_connections(5)
		.connect(&database_url)
		.await?;

	eprintln!("Connected to database");

	let data_dir = Path::new("data/ISSN");

	let files: &[(&str, &str)] = &[
		("WoS-AHCI.csv", "wos"),
		("WoS-ESCI.csv", "wos"),
		("WoS-SCIE.csv", "wos"),
		("WoS-SSCI.csv", "wos"),
		("SCOPUS.csv", "scopus"),
	];

	for (filename, kind) in files {
		let path = data_dir.join(filename);
		if !path.exists() {
			eprintln!("Skipping {filename} (file not found)");
			continue;
		}

		eprintln!("Reading {filename}...");
		let records = reader::read_csv(&path)?;
		eprintln!("  {} records loaded, inserting...", records.len());

		let affected = seeder::seed_records(&pool, &records, kind).await?;
		eprintln!("  Done — {} rows affected", affected);
	}

	eprintln!("\nSeeding complete.");
	Ok(())
}
