use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Default, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetSubfieldsQuery {
    pub field_id: Option<Uuid>,
}
