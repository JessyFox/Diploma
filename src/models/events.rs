pub use super::_entities::events::{ActiveModel, Entity, Model};
use super::_entities::{
    client_ids,
    events::{Column, Relation},
};
use crate::common::result::ApiResult;
use crate::views::avg_stats::AvgStats;
use crate::views::period_stats::PeriodStats;
use chrono::{NaiveDateTime, NaiveTime};
use loco_rs::model::ModelError;
use migration::Query;
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::{self, Alias, Asterisk, CommonTableExpression, Expr, ExprTrait, Func, FunctionCall, WithClause};
use sea_orm::{
    FromQueryResult, IntoSimpleExpr, JoinType, Order, QueryOrder, QuerySelect, QueryTrait,
};
use seaography::CustomInputType;

pub type Events = Entity;

#[derive(Clone)]
pub struct Period {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

impl From<PeriodType> for Period {
    fn from(value: PeriodType) -> Self {
        match value {
            PeriodType::Day(p) => p.into(),
            PeriodType::Month(p) => p.into(),
            PeriodType::Year(p) => p.into(),
        }
    }
}

impl From<Day> for Period {
    fn from(value: Day) -> Self {
        let start = value.start.and_time(NaiveTime::MIN);
        let end = value.end.and_time(NaiveTime::MIN);
        Self { start, end }
    }
}

impl From<Month> for Period {
    fn from(value: Month) -> Self {
        let start = value.start.and_time(NaiveTime::MIN);
        let end = value.end.and_time(NaiveTime::MIN);
        Self { start, end }
    }
}

impl From<Year> for Period {
    fn from(value: Year) -> Self {
        let start = value.start.and_time(NaiveTime::MIN);
        let end = value.end.and_time(NaiveTime::MIN);
        Self { start, end }
    }
}

#[derive(Clone, CustomInputType)]
pub struct Day {
    start: Date,
    end: Date,
}

#[derive(Clone, CustomInputType)]
pub struct Month {
    start: Date,
    end: Date,
}

#[derive(Clone, CustomInputType)]
pub struct Year {
    start: Date,
    end: Date,
}

#[derive(Clone, CustomInputType)]
pub enum PeriodType {
    Day(Day),
    Month(Month),
    Year(Year),
}

#[derive(Iden)]
#[iden = "SUBSTR"]
pub struct Substr;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        Ok(self)
    }
}

fn substr_expr(col: Expr, pos1: i64, pos2: i64) -> FunctionCall {
    Func::cust(Substr).args([
        col,
        Expr::Constant(pos1.into()),
        Expr::Constant(pos2.into()),
    ])
}

fn cast_as_text(col: Expr) -> FunctionCall {
    Func::cast_as(col, Alias::new("TEXT"))
}

// implement your read-oriented logic here
impl Model {
    fn get_period_query(period_type: &PeriodType, event_type_id: i64) -> Select<Entity> {
        let substr_len = match period_type {
            PeriodType::Day(_) => 10,
            PeriodType::Month(_) => 7,
            PeriodType::Year(_) => 4,
        };

        let trigger_col = Expr::col(Column::TriggeredAt);
        let created_col = Expr::col(client_ids::Column::CreatedAt);
        let client_id_col = Expr::col(Column::ClientId);

        let period_expr = substr_expr(
            cast_as_text(trigger_col.clone()).into_simple_expr(),
            1,
            substr_len,
        );

        let eq_expr = substr_expr(cast_as_text(created_col.clone()).into_simple_expr(), 1, 10).eq(
            substr_expr(cast_as_text(trigger_col.clone()).into_simple_expr(), 1, 10),
        );

        let case_expr = Expr::case(eq_expr, client_id_col.clone());

        let period: Period = period_type.clone().into();

        Entity::find()
            .select_only()
            .expr_as(period_expr.clone(), "period")
            .expr_as(Func::count(Expr::col(Asterisk)), "all_visits")
            .expr_as(Func::count_distinct(client_id_col), "visitor_count")
            .expr_as(Func::count_distinct(case_expr), "unique_visitors")
            .join(JoinType::InnerJoin, Relation::ClientIds.def())
            .filter(trigger_col.clone().gte(period.start))
            .filter(trigger_col.lt(period.end))
            .filter(Column::EventTypeId.eq(event_type_id))
            .group_by(period_expr)
            .order_by(Expr::col(Alias::new("period")), Order::Asc)
            .to_owned()
    }

    fn get_global_stats(period_type: &PeriodType, event_type_id: i64) -> Select<Entity> {
        let trigger_col = Expr::col(Column::TriggeredAt);
        let created_col = Expr::col(client_ids::Column::CreatedAt);
        let client_id_col = Expr::col(Column::ClientId);
        let eq_expr = substr_expr(cast_as_text(created_col.clone()).into_simple_expr(), 1, 10).eq(
            substr_expr(cast_as_text(trigger_col.clone()).into_simple_expr(), 1, 10),
        );
        let case_expr = Expr::case(eq_expr, client_id_col.clone());
        let period: Period = period_type.clone().into();
        Entity::find()
            .select_only()
            .expr_as(Func::count_distinct(client_id_col.clone()), "total_visitors")
            .expr_as(Func::count_distinct(case_expr), "total_unique")
            .join(JoinType::InnerJoin, Relation::ClientIds.def())
            .filter(trigger_col.clone().gte(period.start))
            .filter(trigger_col.lt(period.end))
            .filter(Column::EventTypeId.eq(event_type_id))
            .to_owned()
    }

    pub async fn get_period_stats<C: ConnectionTrait>(
        db: &C,
        period_type: &PeriodType,
        event_type_id: i64,
    ) -> loco_rs::Result<Vec<PeriodStats>> {
        let result: Vec<PeriodStats> = Self::get_period_query(period_type, event_type_id)
            .into_model()
            .all(db)
            .await?;
        Ok(result)
    }

    pub async fn get_avg_stats<C: ConnectionTrait>(
        db: &C,
        period_type: &PeriodType,
        event_type_id: i64,
    ) -> ApiResult<AvgStats> {
        let stats_query = Self::get_period_query(period_type, event_type_id).into_query();
        let global_stats_query = Self::get_global_stats(period_type, event_type_id).into_query();
        let stats_cte = CommonTableExpression::new()
            .query(stats_query)
            .table_name(Alias::new("period_stats"))
            .to_owned();
        let global_cte = CommonTableExpression::new()
            .query(global_stats_query)
            .table_name(Alias::new("global_stats"))
            .to_owned();
        let with = WithClause::new()
            .cte(stats_cte)
            .cte(global_cte)
            .to_owned();
        let result_query = Query::select()
            .expr(Expr::col(Alias::new("total_unique")))
            .expr(Expr::col(Alias::new("total_visitors")))
            .expr_as(
                Func::sum(Expr::col(Alias::new("all_visits"))),
                "total_visits",
            )
            .expr_as(
                Func::round_with_precision(Func::avg(Expr::col(Alias::new("all_visits"))), 1),
                "avg_visits",
            )
            .expr_as(
                Func::round_with_precision(Func::avg(Expr::col(Alias::new("visitor_count"))), 1),
                "avg_visitors",
            )
            .expr_as(
                Func::round_with_precision(Func::avg(Expr::col(Alias::new("unique_visitors"))), 1),
                "avg_unique",
            )
            .expr_as(Func::count(Expr::col(Asterisk)), "entries_count")
            .from(Alias::new("period_stats"))
            .cross_join(Alias::new("global_stats"))
            .to_owned();
        let result_query = result_query
            .with(with)
            .to_owned();
        let stmt = db.get_database_backend().build(&result_query);
        let result = AvgStats::find_by_statement(stmt)
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        Ok(result)
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
