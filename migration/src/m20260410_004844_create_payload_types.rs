use crate::m20260410_004554_create_value_types::ValueTypes;
use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.create_table(
            Table::create()
                .table(PayloadTypes::Table)
                .if_not_exists()
                .col(pk_auto(PayloadTypes::Id))
                .col(string_len_uniq(PayloadTypes::CodeName, 20))
                .col(string_len_null(PayloadTypes::Description, 100))
                .col(integer(PayloadTypes::ValueTypeId))
                .foreign_key(
                    ForeignKey::create()
                        .from(PayloadTypes::Table, PayloadTypes::ValueTypeId)
                        .to(ValueTypes::Table, ValueTypes::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(PayloadTypes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum PayloadTypes {
    Table,
    Id,
    CodeName,
    Description,
    ValueTypeId,
}
