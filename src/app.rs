use crate::models::client_ids;
use crate::{
    controllers,
    initializers::{
        graphql::GraphQLInitializer, swagger_ui::SwaggerUiInitializer,
        ua_parser::UAParserInitializer,
    },
    models::_entities::{refresh_tokens, users},
    tasks::refresh_tokens::RefreshTokens,
};
use async_trait::async_trait;
use loco_rs::{
    app::{AppContext, Hooks, Initializer},
    bgworker::Queue,
    boot::{create_app, BootResult, StartMode},
    config::Config,
    controller::AppRoutes,
    db::{self, truncate_table},
    environment::Environment,
    task::Tasks,
    Result,
};
use migration::Migrator;
use std::path::Path;

pub struct App;
#[async_trait]
impl Hooks for App {
    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    async fn boot(
        mode: StartMode,
        environment: &Environment,
        config: Config,
    ) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment, config).await
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![
            Box::new(SwaggerUiInitializer),
            Box::new(GraphQLInitializer),
            Box::new(UAParserInitializer),
        ])
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes() // controller routes below
            .prefix("/api")
            .add_route(controllers::auth::routes())
            .add_route(controllers::graphql::routes())
            .add_route(controllers::events::routes())
            .add_route(controllers::client_ids::routes())
    }

    async fn connect_workers(_ctx: &AppContext, _queue: &Queue) -> Result<()> {
        Ok(())
    }

    fn register_tasks(tasks: &mut Tasks) {
        tasks.register(RefreshTokens);
        // tasks-inject (do not remove)
    }
    async fn truncate(ctx: &AppContext) -> Result<()> {
        truncate_table(&ctx.db, users::Entity).await?;
        truncate_table(&ctx.db, refresh_tokens::Entity).await?;
        truncate_table(&ctx.db, client_ids::Entity).await?;
        Ok(())
    }
    async fn seed(ctx: &AppContext, base: &Path) -> Result<()> {
        db::seed::<users::ActiveModel>(&ctx.db, &base.join("users.yaml").display().to_string())
            .await?;
        db::seed::<refresh_tokens::ActiveModel>(
            &ctx.db,
            &base.join("refresh_tokens.yaml").display().to_string(),
        )
        .await?;
        db::seed::<client_ids::ActiveModel>(
            &ctx.db,
            &base.join("client_ids.yaml").display().to_string(),
        )
        .await?;
        Ok(())
    }
}
