use crate::m20260410_003534_create_event_types::EventTypes;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Query::insert()
            .into_table(EventTypes::Table)
            .columns([EventTypes::Id, EventTypes::CodeName])
            .values_panic([1.into(), "site".into()])
            .values_panic([2.into(), "auds".into()])
            .values_panic([3.into(), "ways".into()])
            .values_panic([4.into(), "plans".into()])
            .to_owned();

        m.exec_stmt(stmt).await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Query::delete()
            .from_table(EventTypes::Table)
            .and_where(Expr::col(EventTypes::Id).is_in(1..=4))
            .to_owned();

        m.exec_stmt(stmt).await?;

        Ok(())
    }
}
