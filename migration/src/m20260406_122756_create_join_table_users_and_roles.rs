use crate::m20220101_000001_users::Users;
use crate::m20260404_225136_roles::Roles;
use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.create_table(
            Table::create()
                .table(UserRoles::Table)
                .if_not_exists()
                .col(integer(UserRoles::UserId))
                .col(integer(UserRoles::RoleId))
                .foreign_key(
                    ForeignKey::create()
                        .from(UserRoles::Table, UserRoles::UserId)
                        .to(Users::Table, Users::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .from(UserRoles::Table, UserRoles::RoleId)
                        .to(Roles::Table, Roles::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .primary_key(
                    Index::create()
                        .col(UserRoles::UserId)
                        .col(UserRoles::RoleId),
                )
                .to_owned(),
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(UserRoles::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserRoles {
    Table,
    UserId,
    RoleId,
}
