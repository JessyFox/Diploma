use crate::common::errors::ApiError;
use axum::response::{IntoResponse, Response};
use loco_rs::controller::Json;
use serde::Serialize;

pub type ApiResult<T, E = ApiError> = Result<T, E>;

pub fn json<T: Serialize>(t: T) -> ApiResult<Response> {
    Ok(Json(t).into_response())
}
