#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use crate::common::result::{json, ApiResult};
use crate::models::client_ids;
use crate::views::client_id::NewClientResponse;
use loco_rs::prelude::*;

#[utoipa::path(
    get,
    path = "/api/client_ids",
    responses(
        (status = 200, description = "Login successful", body = NewClientResponse),
    )
)]
#[debug_handler]
pub async fn create_new(State(ctx): State<AppContext>) -> ApiResult<Response> {
    let resp: NewClientResponse = client_ids::ActiveModel::create_new(&ctx.db).await?.into();
    json(resp)
}

pub fn routes() -> Routes {
    Routes::new().prefix("client_ids").add("/", get(create_new))
}
