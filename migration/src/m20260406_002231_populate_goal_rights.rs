use crate::m20260404_225854_create_join_table_goals_and_rights::GoalRights;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Query::insert()
            .into_table(GoalRights::Table)
            .columns([GoalRights::Id, GoalRights::GoalId, GoalRights::RightId])
            .values_panic([1.into(), 1.into(), 1.into()])
            .values_panic([2.into(), 2.into(), 1.into()])
            .values_panic([3.into(), 3.into(), 1.into()])
            .values_panic([4.into(), 3.into(), 2.into()])
            .values_panic([5.into(), 3.into(), 3.into()])
            .values_panic([6.into(), 3.into(), 4.into()])
            .values_panic([7.into(), 4.into(), 1.into()])
            .values_panic([8.into(), 4.into(), 2.into()])
            .values_panic([9.into(), 4.into(), 3.into()])
            .values_panic([10.into(), 4.into(), 4.into()])
            .values_panic([11.into(), 4.into(), 5.into()])
            .values_panic([12.into(), 5.into(), 1.into()])
            .values_panic([13.into(), 5.into(), 3.into()])
            .values_panic([14.into(), 6.into(), 1.into()])
            .values_panic([15.into(), 6.into(), 2.into()])
            .values_panic([16.into(), 6.into(), 3.into()])
            .values_panic([17.into(), 6.into(), 4.into()])
            .values_panic([18.into(), 7.into(), 1.into()])
            .values_panic([19.into(), 7.into(), 3.into()])
            .values_panic([20.into(), 8.into(), 1.into()])
            .values_panic([21.into(), 8.into(), 2.into()])
            .values_panic([22.into(), 8.into(), 3.into()])
            .values_panic([23.into(), 8.into(), 4.into()])
            .values_panic([24.into(), 9.into(), 3.into()])
            .values_panic([25.into(), 10.into(), 1.into()])
            .values_panic([26.into(), 10.into(), 3.into()])
            .values_panic([27.into(), 11.into(), 1.into()])
            .values_panic([28.into(), 11.into(), 3.into()])
            .to_owned();

        m.exec_stmt(stmt).await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Query::delete()
            .from_table(GoalRights::Table)
            .and_where(Expr::col(GoalRights::Id).is_in(1..=28))
            .to_owned();

        m.exec_stmt(stmt).await?;

        Ok(())
    }
}
