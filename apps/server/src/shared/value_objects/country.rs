use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone)]
pub struct Country {
    pub code: String,
    pub _name: String,
}

impl<'de> Deserialize<'de> for Country {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let full_string = String::deserialize(deserializer)?;
        let parts = full_string.splitn(2, '-').collect::<Vec<&str>>();

        if parts.len() != 2 {
            return Err(serde::de::Error::custom(format!(
                "Invalid country format: '{full_string}'. Expected format 'CODE - NAME'",
            )));
        }

        let code = parts[0].trim().to_string();
        let name = parts[1].trim().to_string();

        Ok(Country { code, _name: name })
    }
}
