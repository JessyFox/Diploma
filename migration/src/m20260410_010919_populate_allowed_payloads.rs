use crate::m20260410_005410_create_allowed_payloads::AllowedPayloads;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Query::insert()
            .into_table(AllowedPayloads::Table)
            .columns([AllowedPayloads::EventTypeId, AllowedPayloads::PayloadTypeId])
            .values_panic([1.into(), 1.into()])
            .values_panic([2.into(), 2.into()])
            .values_panic([2.into(), 3.into()])
            .values_panic([3.into(), 4.into()])
            .values_panic([3.into(), 5.into()])
            .values_panic([4.into(), 6.into()])
            .to_owned();

        m.exec_stmt(stmt).await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Query::delete()
            .from_table(AllowedPayloads::Table)
            .and_where(Expr::col(AllowedPayloads::EventTypeId).is_in(1..=4))
            .and_where(Expr::col(AllowedPayloads::PayloadTypeId).is_in(1..=6))
            .to_owned();

        m.exec_stmt(stmt).await?;

        Ok(())
    }
}
