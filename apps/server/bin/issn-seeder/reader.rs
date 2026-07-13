use std::path::Path;

use csv::ReaderBuilder;

#[derive(Debug)]
pub struct IssnRecord {
	pub issn: Option<String>,
	pub eissn: Option<String>,
}

fn normalize(value: &str) -> Option<String> {
	let cleaned = value.trim().replace('-', "").to_uppercase();
	if cleaned.is_empty() {
		None
	} else {
		Some(cleaned)
	}
}

pub fn read_csv(path: &Path) -> Result<Vec<IssnRecord>, Box<dyn std::error::Error>> {
	let mut reader = ReaderBuilder::new()
		.has_headers(true)
		.trim(csv::Trim::All)
		.from_path(path)?;

	let mut records = Vec::new();

	for result in reader.records() {
		let row = result?;
		let issn = normalize(&row[0]);
		let eissn = normalize(&row[1]);

		if issn.is_none() && eissn.is_none() {
			continue;
		}

		records.push(IssnRecord { issn, eissn });
	}

	Ok(records)
}
