use serde::{Deserialize, Deserializer, de::Error as DeError};
use std::ops::Deref;

#[derive(Debug, Clone, Copy)]
pub struct CLf64(f64);

impl<'de> Deserialize<'de> for CLf64 {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s = String::deserialize(deserializer)?;
		let s = s.replace(',', ".");
		let value = s.parse::<f64>().map_err(DeError::custom)?;

		Ok(Self(value))
	}
}

impl Deref for CLf64 {
	type Target = f64;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
