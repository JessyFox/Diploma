use crate::m20260410_004844_create_payload_types::PayloadTypes;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Query::insert()
            .into_table(PayloadTypes::Table)
            .columns([
                PayloadTypes::Id,
                PayloadTypes::CodeName,
                PayloadTypes::ValueTypeId,
            ])
            .values_panic([1.into(), "endpoint".into(), 3.into()])
            .values_panic([2.into(), "auditory_id".into(), 3.into()])
            .values_panic([3.into(), "success".into(), 2.into()])
            .values_panic([4.into(), "start_id".into(), 3.into()])
            .values_panic([5.into(), "end_id".into(), 3.into()])
            .values_panic([6.into(), "plan_id".into(), 3.into()])
            .to_owned();

        m.exec_stmt(stmt).await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Query::delete()
            .from_table(PayloadTypes::Table)
            .and_where(Expr::col(PayloadTypes::Id).is_in(1..=6))
            .to_owned();

        m.exec_stmt(stmt).await?;

        Ok(())
    }
}
