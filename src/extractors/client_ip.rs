use axum::extract::{ConnectInfo, FromRequestParts};
use axum::http::request::Parts;
use std::net::{IpAddr, SocketAddr};

pub struct ClientIp(pub IpAddr);

impl<S> FromRequestParts<S> for ClientIp
where
    S: Send + Sync,
{
    type Rejection = axum::http::StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(connect_info) = parts.extensions.get::<ConnectInfo<SocketAddr>>() {
            return Ok(Self(connect_info.ip()));
        }
        Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
    }
}
