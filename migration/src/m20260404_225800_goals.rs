use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.create_table(
            Table::create()
                .table(Goals::Table)
                .if_not_exists()
                .col(pk_auto(Goals::Id))
                .col(string_uniq(Goals::Name))
                .to_owned(),
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(Goals::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Goals {
    Table,
    Id,
    Name,
}
