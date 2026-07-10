use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Default, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetFieldsQuery {
    pub domain_id: Option<Uuid>,
}
