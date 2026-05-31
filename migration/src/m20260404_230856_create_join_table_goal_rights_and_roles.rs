use crate::m20260404_225136_roles::Roles;
use crate::m20260404_225854_create_join_table_goals_and_rights::GoalRights;
use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.create_table(
            Table::create()
                .table(GoalRightRoles::Table)
                .if_not_exists()
                .col(integer(GoalRightRoles::GoalRightId))
                .col(integer(GoalRightRoles::RoleId))
                .foreign_key(
                    ForeignKey::create()
                        .from(GoalRightRoles::Table, GoalRightRoles::GoalRightId)
                        .to(GoalRights::Table, GoalRights::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .from(GoalRightRoles::Table, GoalRightRoles::RoleId)
                        .to(Roles::Table, Roles::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .primary_key(
                    Index::create()
                        .col(GoalRightRoles::RoleId)
                        .col(GoalRightRoles::GoalRightId),
                )
                .to_owned(),
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(GoalRightRoles::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum GoalRightRoles {
    Table,
    GoalRightId,
    RoleId,
}
