use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.create_table(
            Table::create()
                .table(ClientIds::Table)
                .if_not_exists()
                .col(pk_auto(ClientIds::Id))
                .col(uuid_uniq(ClientIds::Ident))
                .col(
                    timestamp_with_time_zone(ClientIds::CreatedAt)
                        .default(Expr::current_timestamp()),
                )
                .to_owned(),
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(ClientIds::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ClientIds {
    Table,
    Id,
    Ident,
    CreatedAt,
}
