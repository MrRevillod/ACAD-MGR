use regex::Regex;
use std::sync::LazyLock;

static ORCID_URL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
	Regex::new(r"^https://orcid\.org/\d{4}-\d{4}-\d{4}-\d{3}[\dX]$").expect("regex inválida")
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Orcid(String);

impl Orcid {
	pub fn normalize(raw: &str) -> Option<String> {
		let trimmed = raw.trim();

		if ORCID_URL_REGEX.is_match(trimmed) {
			Some(trimmed.to_string())
		} else {
			None
		}
	}
}
