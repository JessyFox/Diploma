use crate::m20260410_004425_events::Events;
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
                .table(Payloads::Table)
                .if_not_exists()
                .col(pk_auto(Payloads::Id))
                .col(integer(Payloads::EventId))
                .col(integer(Payloads::TypeId))
                .col(string_len(Payloads::Value, 50))
                .foreign_key(
                    ForeignKey::create()
                        .from(Payloads::Table, Payloads::EventId)
                        .to(Events::Table, Events::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .from(Payloads::Table, Payloads::TypeId)
                        .to(PayloadTypes::Table, PayloadTypes::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(Payloads::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Payloads {
    Table,
    Id,
    EventId,
    TypeId,
    Value,
}
