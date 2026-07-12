use std::error::Error;

mod fetcher;
mod progress;
mod seeder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let api_key = std::env::var("OPENALEX_API_KEY")?;
	let database_url = std::env::var("LOCAL_POSTGRES_DATABASE_URL")
		.or_else(|_| std::env::var("POSTGRES_DATABASE_URL"))?;

	let pool = sqlx::postgres::PgPoolOptions::new()
		.max_connections(5)
		.connect(&database_url)
		.await?;

	eprintln!("Connected to database");

	seeder::seed_unknown_taxonomy(&pool).await?;
	eprintln!("✓ Unknown taxonomy seeded");

	let client = fetcher::OpenAlexFetcher::new(&api_key);
	let mut progress = progress::Progress::new(4);

	let domains = client.fetch_all_domains().await?;
	let n = seeder::seed_domains(&pool, &domains).await?;
	progress.tick(&format!("Domains: {n} inserted"));

	let fields = client.fetch_all_fields().await?;
	let n = seeder::seed_fields(&pool, &fields).await?;
	progress.tick(&format!("Fields: {n} inserted"));

	let subfields = client.fetch_all_subfields().await?;
	let n = seeder::seed_subfields(&pool, &subfields).await?;
	progress.tick(&format!("Subfields: {n} inserted"));

	let topics = client.fetch_all_topics().await?;
	let n = seeder::seed_topics(&pool, &topics).await?;
	progress.tick(&format!("Topics: {n} inserted"));

	progress.print_summary();
	Ok(())
}
