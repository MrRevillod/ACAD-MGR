use serde::Deserialize;
use validator::Validate;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetKeywordsQuery {}
