use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.create_table(
            Table::create()
                .table(Rights::Table)
                .if_not_exists()
                .col(pk_auto(Rights::Id))
                .col(string_uniq(Rights::Name))
                .to_owned(),
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(Rights::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Rights {
    Table,
    Id,
    Name,
}
