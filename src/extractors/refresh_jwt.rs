use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use loco_rs::{
    app::AppContext, auth, controller::extractor::auth::extract_token_from_cookie, Error,
};
use serde::{Deserialize, Serialize};

use crate::common::settings::{Refresh, Settings};

#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshJWT {
    pub claims: auth::jwt::UserClaims,
}

impl<S> FromRequestParts<S> for RefreshJWT
where
    AppContext: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Error> {
        extract_jwt_from_request_parts(parts, state)
    }
}

/// extract a [JWT] token from request parts, using a non-mutable reference to the [Parts]
///
/// # Errors
/// Return an error when JWT token not configured or when the token is not valid
pub fn extract_jwt_from_request_parts<S>(parts: &Parts, state: &S) -> Result<RefreshJWT, Error>
where
    AppContext: FromRef<S>,
    S: Send + Sync,
{
    let ctx: AppContext = AppContext::from_ref(state); // change to ctx

    let refresh_config = get_refresh_from_config(&ctx)?;

    let token = extract_token(&refresh_config, parts)?;

    match auth::jwt::JWT::new(&refresh_config.secret).validate(&token) {
        Ok(claims) => Ok(RefreshJWT {
            claims: claims.claims,
        }),
        Err(err) => {
            tracing::error!("Refresh token validation error: {}", err);
            Err(Error::Unauthorized("token is not valid".to_string()))
        }
    }
}

/// extract refresh token from context configuration
///
/// # Errors
/// Return an error when JWT token not configured
pub fn get_refresh_from_config(ctx: &AppContext) -> loco_rs::Result<Refresh> {
    ctx.config
        .settings
        .as_ref()
        .map(Settings::from_json)
        .ok_or_else(|| Error::string("Settings not configured"))?
        .map(|v| v.refresh)?
        .ok_or_else(|| Error::string("Refresh token is not configured"))
}

/// extract token from the configured jwt location settings
///
/// # Errors
///
/// Returns an error when the token cannot be extracted from any of the configured locations,
/// such as missing headers, invalid formats, or inaccessible request data.
pub fn extract_token(refresh_config: &Refresh, parts: &Parts) -> loco_rs::Result<String> {
    if let Ok(token) = extract_token_from_cookie(&refresh_config.name, parts) {
        return Ok(token);
    }

    Err(Error::Unauthorized(
        "Refresh token not found in configured location.".to_string(),
    ))
}

impl RefreshJWT {
    /// Extracts field jti from JWT token
    ///
    /// # Errors
    /// When field jti is missing
    pub fn extract_jti(&self) -> loco_rs::Result<uuid::Uuid> {
        let jti = self
            .claims
            .claims
            .get("jti")
            .ok_or_else(|| Error::BadRequest("Invalid refresh token".to_string()))?
            .as_str()
            .ok_or_else(|| Error::BadRequest("Invalid refresh token".to_string()))?
            .to_string();
        uuid::Uuid::parse_str(&jti)
            .map_err(|err| Error::BadRequest(format!("Not valid jti: {err}")))
    }
}
