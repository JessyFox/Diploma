use crate::m20220101_000001_users::Users;
use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.create_table(
            Table::create()
                .table(RefreshTokens::Table)
                .if_not_exists()
                .col(pk_auto(RefreshTokens::Id))
                .col(integer(RefreshTokens::UserId))
                .col(uuid_uniq(RefreshTokens::Jti))
                .col(string_null(RefreshTokens::Browser))
                .col(string_null(RefreshTokens::UserIp))
                .col(boolean(RefreshTokens::Revoked))
                .col(
                    timestamp_with_time_zone(RefreshTokens::CreatedAt)
                        .default(Expr::current_timestamp()),
                )
                .col(
                    timestamp_with_time_zone(RefreshTokens::UpdatedAt)
                        .default(Expr::current_timestamp()),
                )
                .col(timestamp_with_time_zone(RefreshTokens::ExpiredAt))
                .foreign_key(
                    ForeignKey::create()
                        .from(RefreshTokens::Table, RefreshTokens::UserId)
                        .to(Users::Table, Users::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(RefreshTokens::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum RefreshTokens {
    Table,
    Id,
    UserId,
    Jti,
    Browser,
    UserIp,
    Revoked,
    ExpiredAt,
    CreatedAt,
    UpdatedAt,
}
