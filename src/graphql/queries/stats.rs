use crate::models::events::PeriodType;
use crate::models::{event_types, events};
use crate::views::period_stats::PeriodStats;
use async_graphql::Context;
use sea_orm::DatabaseConnection;
use seaography::{CustomFields, CustomInputType};
use crate::views::avg_stats::AvgStats;

#[derive(Clone, CustomInputType)]
pub struct PeriodStatsInput {
    pub endpoint: String,
    pub period_type: PeriodType,
}

pub struct Operations;

#[CustomFields]
impl Operations {
    async fn period_stats(
        ctx: &Context<'_>,
        params: PeriodStatsInput,
    ) -> async_graphql::Result<Vec<PeriodStats>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let id = event_types::Model::get_id_by_name(db, &params.endpoint).await?;
        let data =
            events::Model::get_period_stats(db, &params.period_type, id).await?;
        Ok(data)
    }

    async fn avg_stats(
        ctx: &Context<'_>,
        params: PeriodStatsInput
    ) -> async_graphql::Result<AvgStats> {
        let db = ctx.data::<DatabaseConnection>()?;
        let id = event_types::Model::get_id_by_name(db, &params.endpoint).await?;
        let data = events::Model::get_avg_stats(db, &params.period_type, id).await?;
        Ok(data)
    }
}
