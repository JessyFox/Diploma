use sea_orm::FromQueryResult;
use seaography::CustomOutputType;

#[derive(FromQueryResult, Debug, Clone, CustomOutputType)]
pub struct PeriodStats {
    pub period: String,
    pub all_visits: i64,
    pub visitor_count: i64,
    pub unique_visitors: i64,
}
