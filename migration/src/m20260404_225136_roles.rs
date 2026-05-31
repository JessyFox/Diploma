use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.create_table(
            Table::create()
                .table(Roles::Table)
                .if_not_exists()
                .col(pk_auto(Roles::Id))
                .col(string_uniq(Roles::Name))
                .col(timestamp_with_time_zone(Roles::CreatedAt).default(Expr::current_timestamp()))
                .col(timestamp_with_time_zone(Roles::UpdatedAt).default(Expr::current_timestamp()))
                .to_owned(),
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(Roles::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Roles {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
}
