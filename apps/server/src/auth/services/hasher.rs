use crate::{auth::AuthError, shared::AppResult};
use serde::Deserialize;
use sword::prelude::*;

#[config(key = "hasher")]
#[derive(Clone, Deserialize)]
pub struct HasherConfig {
	pub cost: u32,
}

#[injectable(provider)]
pub struct Hasher {
	cost: u32,
}

impl Hasher {
	pub fn new(config: &HasherConfig) -> Self {
		Self { cost: config.cost }
	}

	pub fn hash(&self, password: &str) -> AppResult<String> {
		let hash = bcrypt::hash(password, self.cost)
			.inspect_err(|e| eprintln!("{e}"))
			.map_err(AuthError::from)?;

		Ok(hash)
	}

	pub fn verify(&self, password: &str, hash: &str) -> AppResult<bool> {
		let is_valid = bcrypt::verify(password, hash)
			.inspect_err(|e| eprintln!("{e}"))
			.map_err(AuthError::from)?;

		Ok(is_valid)
	}
}

impl Default for HasherConfig {
	fn default() -> Self {
		Self { cost: 12 }
	}
}
