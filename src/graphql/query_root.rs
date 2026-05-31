use super::guards::auth::AuthGuard;
use async_graphql::dynamic::{Schema, SchemaError};
use sea_orm::DatabaseConnection;
use seaography::{async_graphql, Builder, BuilderContext, LifecycleHooks, MultiLifecycleHooks};
use std::sync::LazyLock;
use crate::graphql::queries::stats;
use crate::graphql::queries::stats::PeriodStatsInput;
use crate::models::events::{Day, Month, Year, PeriodType};
use crate::views::avg_stats::AvgStats;
use crate::views::period_stats::PeriodStats;

static CONTEXT: LazyLock<BuilderContext> = LazyLock::new(|| BuilderContext {
    hooks: LifecycleHooks::new(MultiLifecycleHooks::default().add(AuthGuard::new())),
    ..Default::default()
});

/// Build GraphQL schema
///
/// # Errors
/// When graphql schema is invalid
pub fn schema(
    database: DatabaseConnection,
    depth: Option<usize>,
    complexity: Option<usize>,
) -> Result<Schema, SchemaError> {
    // Construct GraphQL schema
    let builder = Builder::new(&CONTEXT, database.clone());
    let mut builder = crate::models::_entities::register_entity_modules(builder);
    seaography::register_custom_inputs!(builder, [PeriodStatsInput, Day, Month, Year, PeriodType]);
    seaography::register_custom_outputs!(builder, [PeriodStats, AvgStats]);
    seaography::register_custom_queries!(builder, [stats::Operations]);
    builder
        // Maximum depth of the constructed query
        .set_depth_limit(depth)
        // Maximum complexity of the constructed query
        .set_complexity_limit(complexity)
        .schema_builder()
        // GraphQL schema with database connection
        .data(database)
        .finish()
}
