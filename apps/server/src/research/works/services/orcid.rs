use crate::research::WorksError;
use crate::shared::AppResult;

use chrono::NaiveDate;
use orcid::Client as OrcidApiClient;
use sword::prelude::*;

#[derive(Debug, Clone)]
pub struct OrcidWork {
	pub doi: Option<String>,
	pub publication_date: Option<NaiveDate>,
}

#[injectable(provider)]
pub struct OrcidClient {
	inner: OrcidApiClient,
}

impl OrcidClient {
	pub fn new() -> Self {
		Self {
			inner: OrcidApiClient::new(),
		}
	}

	pub async fn works(&self, orcid: &str) -> AppResult<Vec<OrcidWork>> {
		let bare = orcid.strip_prefix("https://orcid.org/").unwrap_or(orcid);

		let author = self
			.inner
			.author(bare)
			.await
			.map_err(|e| WorksError::Other(format!("ORCID API error: {e}")))?;

		Ok(author
			.works()
			.into_iter()
			.map(|w| {
				let doi = w
					.external_ids
					.iter()
					.find(|(ty, _)| ty == "doi")
					.map(|(_, value)| value.clone());

				let publication_date = w.publication_date.year().and_then(|year| {
					NaiveDate::from_ymd_opt(
						year as i32,
						u32::from(w.publication_date.month().unwrap_or(1)),
						u32::from(w.publication_date.day().unwrap_or(1)),
					)
				});

				OrcidWork {
					doi,
					publication_date,
				}
			})
			.collect())
	}
}
