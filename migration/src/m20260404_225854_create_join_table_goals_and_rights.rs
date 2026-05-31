use crate::m20260404_225800_goals::Goals;
use crate::m20260404_225817_rights::Rights;
use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.create_table(
            Table::create()
                .table(GoalRights::Table)
                .if_not_exists()
                .col(pk_auto(GoalRights::Id))
                .col(integer(GoalRights::GoalId))
                .col(integer(GoalRights::RightId))
                .foreign_key(
                    ForeignKey::create()
                        .from(GoalRights::Table, GoalRights::GoalId)
                        .to(Goals::Table, Goals::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .from(GoalRights::Table, GoalRights::RightId)
                        .to(Rights::Table, Rights::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(GoalRights::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum GoalRights {
    Table,
    Id,
    GoalId,
    RightId,
}
