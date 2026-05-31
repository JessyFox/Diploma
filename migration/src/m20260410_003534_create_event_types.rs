use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.create_table(
            Table::create()
                .table(EventTypes::Table)
                .if_not_exists()
                .col(pk_auto(EventTypes::Id))
                .col(string_len_uniq(EventTypes::CodeName, 20))
                .col(string_len_null(EventTypes::Description, 100))
                .to_owned(),
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(EventTypes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum EventTypes {
    Table,
    Id,
    CodeName,
    Description,
}
