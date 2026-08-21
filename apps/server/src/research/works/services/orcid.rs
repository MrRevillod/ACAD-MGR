use crate::research::WorksError;
use crate::shared::AppResult;

use orcid::Client as OrcidApiClient;
use sword::prelude::*;

#[derive(Debug, Clone)]
pub struct OrcidWork {
	pub doi: Option<String>,
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

				OrcidWork { doi }
			})
			.collect())
	}
}
