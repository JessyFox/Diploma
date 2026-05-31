use crate::common::errors::ApiError;
use crate::common::result::{json, ApiResult};
use crate::extractors::client_ip::ClientIp;
use crate::extractors::refresh_jwt::{get_refresh_from_config, RefreshJWT};
use crate::models::refresh_tokens::ActiveModel;
use crate::views::status::Status;
use crate::{
    common::constants::{EDIT_RIGHT_NAME, USERS_GOAL_NAME},
    models::{
        _entities::{refresh_tokens, users},
        users::LoginParams,
    },
    services::{permissions::PermissionService, ua_parser::UAParser},
    views::auth::{CurrentResponse, LoginResponse},
};
use axum::{
    http::{self, header::USER_AGENT, HeaderMap},
    Json,
};
use chrono::Duration;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ResetParams {
    #[schema(min_length = 8, example = "Str0ngP@ssw0rd!")]
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct RevokeParams {
    #[schema(format = Uuid)]
    pub jti: Uuid,
}

/// reset user password by the given parameters
#[utoipa::path(
    post,
    path = "/api/auth/change_password",
    tag = "auth",
    request_body = ResetParams,
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Password reset successfully", body = inline(())),
        (status = 401, description = "Invalid or expired token"),
        (status = 400, description = "Invalid password format"),
        (status = 500, description = "Internal server error")
    )
)]
#[debug_handler]
async fn reset(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<ResetParams>,
) -> ApiResult<Response> {
    let Ok(user) = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // returning success to the caller
        tracing::info!("reset token not found");

        return json(());
    };
    user.into_active_model()
        .reset_password(&ctx.db, &params.password)
        .await?;

    json(())
}

/// Creates a user login and returns a token
#[utoipa::path(
    post,
    path = "/api/auth/token",
    tag = "auth",
    request_body = LoginParams,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse, 
            headers(
                ("set-cookie" = String, description = "Refresh token cookie (only if scope=long)")
            )
        ),
        (status = 401, description = "Invalid credentials"),
        (status = 400, description = "Missing User-Agent header"),
        (status = 500, description = "Internal server error")
    )
)]
#[debug_handler]
async fn login(
    State(ctx): State<AppContext>,
    headers: HeaderMap,
    SharedStore(ua_parser_service): SharedStore<UAParser>,
    ClientIp(user_ip): ClientIp,
    Json(params): Json<LoginParams>,
) -> ApiResult<Response> {
    let Ok(user) = users::Model::find_by_login(&ctx.db, &params.login).await else {
        tracing::debug!(
            username = params.login,
            "login attempt with non-existent username"
        );
        return Err(ApiError::Unauthorized("Invalid credentials!".to_string()));
    };

    let valid = user.verify_password(&params.password);

    if !valid {
        return Err(ApiError::Unauthorized("Invalid credentials!".to_string()));
    }

    let jwt_secret = ctx.config.get_jwt_config()?;

    let token = user
        .generate_jwt(&jwt_secret.secret, jwt_secret.expiration)
        .or_else(|_| Err(ApiError::Unauthorized("Invalid credentials!".to_string())))?;

    let mut response = json(LoginResponse::new(&user, &token))?;

    if let Some(scope) = params.scope {
        if &scope == "long" {
            let refresh = get_refresh_from_config(&ctx)?;
            let ua_str = headers
                .get(USER_AGENT)
                .and_then(|v| v.to_str().ok())
                .ok_or(ApiError::InternalServerError)?;

            let user_ip = user_ip.to_string();
            tracing::debug!("{}", user_ip);
            let expires_at = chrono::Utc::now().fixed_offset()
                + Duration::seconds(refresh.expiration.cast_signed());
            let cookie = <refresh_tokens::ModelEx as Into<refresh_tokens::Model>>::into(
                ActiveModel::builder()
                    .set_user_id(user.id)
                    .set_browser(Some(ua_parser_service.parse_ua(ua_str)?))
                    .set_user_ip(Some(user_ip))
                    .set_expired_at(expires_at)
                    .set_revoked(false)
                    .insert(&ctx.db)
                    .await?,
            )
            .generate_refresh_token(
                &refresh.secret,
                refresh.expiration,
                &refresh.name,
                &user.pid,
            )?;
            response
                .headers_mut()
                .append(http::header::SET_COOKIE, cookie.to_string().parse()?);
        }
    }
    Ok(response)
}

/// Revoke a refresh token by its JTI
#[utoipa::path(
    post,
    path = "/api/auth/revoke",
    tag = "auth",
    request_body = RevokeParams,
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Token revoked successfully", body = Status),
        (status = 401, description = "Invalid JWT"),
        (status = 403, description = "User lacks permission to revoke others' tokens"),
        (status = 404, description = "Refresh token not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[debug_handler]
async fn revoke(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<RevokeParams>,
) -> ApiResult<Response> {
    let Ok(user) = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await else {
        tracing::debug!(pid = auth.claims.pid, "Invalid auth-token");
        return Err(ApiError::Unauthorized("Invalid credentials!".to_string()));
    };

    let Ok(refresh_token) = refresh_tokens::Model::find_by_jti(&ctx.db, params.jti).await else {
        tracing::debug!(jti = params.jti.to_string(), "No such refresh token");
        return Err(ApiError::NotFound("JTI".to_string()));
    };

    if refresh_token.user_id != user.id {
        let service = PermissionService::new();
        let is_capable = service
            .check_permission(
                &ctx.db,
                &ctx.cache,
                user.id,
                USERS_GOAL_NAME,
                &[EDIT_RIGHT_NAME],
            )
            .await?;
        if !is_capable {
            return Err(ApiError::Forbidden);
        }
    }

    refresh_token
        .into_active_model()
        .revoke_token(&ctx.db)
        .await?;

    json(Status::default())
}

/// Refresh access token using valid refresh cookie
#[utoipa::path(
    get,
    path = "/api/auth/refresh",
    tag = "auth",
    security(("refresh_cookie" = [])),
    responses(
        (status = 200, description = "Tokens refreshed", body = LoginResponse,
            headers(
                ("set-cookie" = String, description = "New refresh token cookie")
            )
        ),
        (status = 401, description = "Invalid or revoked refresh token"),
        (status = 400, description = "Malformed JTI in token"),
        (status = 500, description = "Internal server error")
    )
)]
#[debug_handler]
async fn refresh(refresh: RefreshJWT, State(ctx): State<AppContext>) -> ApiResult<Response> {
    let jti = refresh.extract_jti()?;

    let Ok(user) = users::Model::find_by_pid(&ctx.db, &refresh.claims.pid).await else {
        tracing::debug!(pid = &refresh.claims.pid, "Invalid refresh token");
        return Err(ApiError::Unauthorized("Invalid credentials!".to_string()));
    };

    let Ok(refresh_token) = refresh_tokens::Model::find_by_jti(&ctx.db, jti).await else {
        tracing::debug!(jti = jti.to_string(), "No such jti in database");
        return Err(ApiError::Unauthorized("Invalid refresh token!".to_string()));
    };

    let new_refresh_token = refresh_token
        .into_active_model()
        .rotate_refresh_token(&ctx.db)
        .await?;

    let jwt_config = ctx.config.get_jwt_config()?;

    let token = user
        .generate_jwt(&jwt_config.secret, jwt_config.expiration)
        .or_else(|_| Err(ApiError::Unauthorized("Invalid credentials!".to_string())))?;

    let mut response = json(LoginResponse::new(&user, &token))?;
    let refresh_config = get_refresh_from_config(&ctx)?;
    let refresh_cookie = new_refresh_token.generate_refresh_token(
        &refresh_config.secret,
        refresh_config.expiration,
        &refresh_config.name,
        &user.pid,
    )?;
    response.headers_mut().append(
        http::header::SET_COOKIE,
        refresh_cookie.to_string().parse()?,
    );
    Ok(response)
}

/// Get current authenticated user profile
#[utoipa::path(
    get,
    path = "/api/auth/me",
    tag = "auth",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "User profile retrieved", body = CurrentResponse),
        (status = 401, description = "Invalid or expired JWT"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[debug_handler]
async fn current(auth: auth::JWT, State(ctx): State<AppContext>) -> ApiResult<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    json(CurrentResponse::new(&user))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("auth")
        .add("/token", post(login))
        .add("/change_password", post(reset))
        .add("/me", get(current))
        .add("/revoke", post(revoke))
        .add("/refresh", get(refresh))
}
