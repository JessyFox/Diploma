use crate::services::permissions::{PermissionService, RequestPermissions};
use async_graphql::{
    dynamic::Schema,
    http::{playground_source, GraphQLPlaygroundConfig},
};
use async_graphql_axum::GraphQLRequest;
use loco_rs::prelude::*;
use seaography::async_graphql;

async fn graphql_playground() -> Result<Response> {
    // Setup GraphQL playground web and specify the endpoint for GraphQL resolver
    let config =
        GraphQLPlaygroundConfig::new("/api/graphql").with_header("Authorization", "AUTO_TOKEN");

    let res = playground_source(config).replace(
        r#""Authorization":"AUTO_TOKEN""#,
        r#""Authorization":`Bearer ${localStorage.getItem('auth_token')}`"#,
    );

    Ok(Response::new(res.into()))
}

async fn graphql_handler(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    gql_req: GraphQLRequest,
) -> Result<async_graphql_axum::GraphQLResponse, (axum::http::StatusCode, &'static str)> {
    let user_id = auth
        .claims
        .claims
        .get("id")
        .ok_or_else(|| {
            tracing::debug!("No field \"id\" in JWT Token... Strange!");
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Invalid token",
            )
        })?
        .as_i64()
        .ok_or_else(|| {
            tracing::debug!("No field \"id\" in JWT Token... Strange!");
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Invalid token",
            )
        })?;

    let permissions = {
        let service = PermissionService::new();
        service
            .get_user_permissions(&ctx.db, &ctx.cache, user_id)
            .await
            .map_err(|e| {
                tracing::error!(user_id = user_id, "Failed to load permissions: {e}");
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Permission load error",
                )
            })?
    };

    let mut gql_req = gql_req.into_inner();
    gql_req = gql_req
        .data(seaography::UserContext { user_id })
        .data(RequestPermissions::new(user_id, permissions));

    let schema: Schema = ctx.shared_store.get().ok_or((
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        "GraphQL not setup",
    ))?;
    let res = schema.execute(gql_req).await.into();

    Ok(res)
}

pub fn routes() -> Routes {
    Routes::new()
        // GraphQL route prefix
        .prefix("graphql")
        // Serving the GraphQL playground web
        .add("/", get(graphql_playground))
        // Handling GraphQL request
        .add("/", post(graphql_handler))
}
