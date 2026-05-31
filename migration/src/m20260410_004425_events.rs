use crate::m20260410_003534_create_event_types::EventTypes;
use crate::m20260410_003912_create_client_ids::ClientIds;
use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.create_table(
            Table::create()
                .table(Events::Table)
                .if_not_exists()
                .col(pk_auto(Events::Id))
                .col(integer(Events::ClientId))
                .col(integer(Events::EventTypeId))
                .col(
                    timestamp_with_time_zone(Events::TriggeredAt)
                        .default(Expr::current_timestamp()),
                )
                .foreign_key(
                    ForeignKey::create()
                        .from(Events::Table, Events::ClientId)
                        .to(ClientIds::Table, ClientIds::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .from(Events::Table, Events::EventTypeId)
                        .to(EventTypes::Table, EventTypes::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(Events::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Events {
    Table,
    Id,
    ClientId,
    EventTypeId,
    TriggeredAt,
}
