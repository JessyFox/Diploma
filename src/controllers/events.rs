#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]

use crate::common::errors::ApiError;
use crate::common::result::{json, ApiResult};
use crate::models::{allowed_payloads, client_ids, events, payload_types, payloads};
use crate::views::status::Status;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct InsertParams {
    #[schema(example = 1)]
    pub event_type_id: i64,
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub client_id: Uuid,
    #[schema(example = json!({ "1": "100", "2": "true", "3": "custom text" }))]
    pub payloads: HashMap<i64, String>,
}

#[utoipa::path(
    put,
    path = "/api/events",
    request_body = InsertParams,
    responses(
        (status = 200, description = "Event successfully registered", body = Status),
        (status = 400, description = "Invalid payload types for event or validation error"),
        (status = 422, description = "Validation error: Invalid value type"),
        (status = 500, description = "Internal error")
    ),
    tag = "Events"
)]
#[debug_handler]
pub async fn insert(
    State(ctx): State<AppContext>,
    Json(params): Json<InsertParams>,
) -> ApiResult<Response> {
    let client = client_ids::Model::find_by_uuid(&ctx.db, params.client_id).await?;

    let payload_type_ids: Vec<i64> = params.payloads.keys().copied().collect();

    if !allowed_payloads::Entity::is_allowed(&ctx.db, params.event_type_id, &payload_type_ids)
        .await?
    {
        return Err(ApiError::BadRequest(
            "This payload types not allowed for that event".to_string(),
        ));
    }

    let type_models = payload_types::Model::find_by_ids(&ctx.db, &payload_type_ids).await?;
    let type_map: HashMap<i64, &payload_types::Model> =
        type_models.iter().map(|m| (m.id, m)).collect();

    // 4. Валидируем значения (HashMap даёт O(1) доступ, порядок не важен)
    for (type_id, value) in &params.payloads {
        if let Some(model) = type_map.get(type_id) {
            if !model.validate(value) {
                return Err(ApiError::UnprocessableEntity(
                    "Invalid payload value type".to_string(),
                ));
            }
        }
    }

    let txn = ctx.db.begin().await?;

    let event = events::ActiveModel::builder()
        .set_client_id(client.id)
        .set_event_type_id(params.event_type_id)
        .insert(&txn)
        .await?;

    let payload_records: Vec<payloads::ActiveModel> = params
        .payloads
        .into_iter()
        .map(|(type_id, value)| {
            let am: payloads::ActiveModel = payloads::ActiveModel::builder()
                .set_event_id(event.id)
                .set_type_id(type_id)
                .set_value(value)
                .into();
            am.validate().map(|()| am)
        })
        .collect::<Result<_, _>>()?;

    payloads::Entity::insert_many(payload_records)
        .exec(&txn)
        .await?;

    txn.commit().await?;

    json(Status::default())
}

pub fn routes() -> Routes {
    Routes::new().prefix("events").add("/", put(insert))
}
