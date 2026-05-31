use sea_orm::FromQueryResult;
use seaography::CustomOutputType;

#[derive(Clone, CustomOutputType, FromQueryResult)]
pub struct AvgStats {
    total_visits: i64,
    total_unique: i64,
    total_visitors: i64,
    avg_visits: f64,
    avg_unique: f64,
    avg_visitors: f64,
    entries_count: i64,
}
