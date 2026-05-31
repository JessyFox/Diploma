use crate::models::client_ids::Model;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct NewClientResponse {
    ident: uuid::Uuid,
    created_at: DateTime<chrono::FixedOffset>,
}

impl From<Model> for NewClientResponse {
    fn from(value: Model) -> Self {
        Self {
            ident: value.ident,
            created_at: value.created_at,
        }
    }
}
