use crate::m20260410_003534_create_event_types::EventTypes;
use crate::m20260410_004844_create_payload_types::PayloadTypes;
use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.create_table(
            Table::create()
                .table(AllowedPayloads::Table)
                .if_not_exists()
                .col(integer(AllowedPayloads::EventTypeId))
                .col(integer(AllowedPayloads::PayloadTypeId))
                .foreign_key(
                    ForeignKey::create()
                        .from(AllowedPayloads::Table, AllowedPayloads::EventTypeId)
                        .to(EventTypes::Table, EventTypes::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .from(AllowedPayloads::Table, AllowedPayloads::PayloadTypeId)
                        .to(PayloadTypes::Table, PayloadTypes::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .primary_key(
                    Index::create()
                        .col(AllowedPayloads::EventTypeId)
                        .col(AllowedPayloads::PayloadTypeId),
                )
                .to_owned(),
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(AllowedPayloads::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AllowedPayloads {
    Table,
    EventTypeId,
    PayloadTypeId,
}
