use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.create_table(
            Table::create()
                .table(ValueTypes::Table)
                .if_not_exists()
                .col(pk_auto(ValueTypes::Id))
                .col(string_len_uniq(ValueTypes::Name, 20))
                .col(string_len_null(ValueTypes::Description, 100))
                .to_owned(),
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(ValueTypes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ValueTypes {
    Table,
    Id,
    Name,
    Description,
}
