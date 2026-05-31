use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.create_table(
            Table::create()
                .table(Users::Table)
                .if_not_exists()
                .col(pk_auto(Users::Id))
                .col(uuid_uniq(Users::Pid))
                .col(string_uniq(Users::Login))
                .col(string_uniq(Users::Email))
                .col(string(Users::Password))
                .col(string(Users::Name))
                .col(boolean(Users::IsActive))
                .col(timestamp_with_time_zone(Users::CreatedAt).default(Expr::current_timestamp()))
                .col(timestamp_with_time_zone(Users::UpdatedAt).default(Expr::current_timestamp()))
                .to_owned(),
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Users {
    Table,
    Id,
    Pid,
    Login,
    Email,
    Password,
    Name,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
