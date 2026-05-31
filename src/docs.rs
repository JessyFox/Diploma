#![allow(clippy::needless_for_each)]
use crate::controllers::auth::{
    ResetParams, RevokeParams, __path_current, __path_login, __path_refresh, __path_reset,
    __path_revoke,
};
use crate::controllers::client_ids::{__path_create_new};
use crate::controllers::events::{InsertParams, __path_insert};
use crate::models::users::LoginParams;
use crate::views::auth::{CurrentResponse, LoginResponse};
use crate::views::status::Status;
use utoipa::OpenApi;
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify,
};

// Модификатор для добавления Bearer Auth в документацию
pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
            components.add_security_scheme(
                "refresh_cookie",
                SecurityScheme::ApiKey(utoipa::openapi::security::ApiKey::Cookie(
                    utoipa::openapi::security::ApiKeyValue::new("refresh_token"),
                )),
            );
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        login,
        reset,
        current,
        revoke,
        refresh,
        insert,
        create_new
    ),
    components(
        schemas(
            LoginParams,
            ResetParams,
            RevokeParams,
            LoginResponse,
            CurrentResponse,
            Status,
            InsertParams,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "auth", description = "Authentication and token management")
    )
)]
pub struct ApiDoc;
