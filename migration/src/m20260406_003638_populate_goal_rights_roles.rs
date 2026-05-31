use crate::m20260404_230856_create_join_table_goal_rights_and_roles::GoalRightRoles;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Query::insert()
            .into_table(GoalRightRoles::Table)
            .columns([GoalRightRoles::RoleId, GoalRightRoles::GoalRightId])
            .values_from_panic([
                [1.into(), 1.into()],
                [1.into(), 2.into()],
                [1.into(), 3.into()],
                [1.into(), 4.into()],
                [1.into(), 5.into()],
                [1.into(), 6.into()],
                [1.into(), 7.into()],
                [1.into(), 8.into()],
                [1.into(), 9.into()],
                [1.into(), 10.into()],
                [1.into(), 11.into()],
                [1.into(), 12.into()],
                [1.into(), 13.into()],
                [1.into(), 14.into()],
                [1.into(), 15.into()],
                [1.into(), 16.into()],
                [1.into(), 17.into()],
                [1.into(), 18.into()],
                [1.into(), 19.into()],
                [1.into(), 20.into()],
                [1.into(), 21.into()],
                [1.into(), 22.into()],
                [1.into(), 23.into()],
                [1.into(), 24.into()],
                [1.into(), 25.into()],
                [1.into(), 26.into()],
                [1.into(), 27.into()],
                [1.into(), 28.into()],
            ])
            .to_owned();

        m.exec_stmt(stmt).await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Query::delete()
            .from_table(GoalRightRoles::Table)
            .and_where(Expr::col(GoalRightRoles::RoleId).eq(1))
            .and_where(Expr::col(GoalRightRoles::GoalRightId).is_in(1..=28))
            .to_owned();

        m.exec_stmt(stmt).await?;

        Ok(())
    }
}
