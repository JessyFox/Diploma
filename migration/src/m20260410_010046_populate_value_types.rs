use crate::m20260410_004554_create_value_types::ValueTypes;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Query::insert()
            .into_table(ValueTypes::Table)
            .columns([ValueTypes::Id, ValueTypes::Name])
            .values_panic([1.into(), "int".into()])
            .values_panic([2.into(), "bool".into()])
            .values_panic([3.into(), "string".into()])
            .to_owned();

        m.exec_stmt(stmt).await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Query::delete()
            .from_table(ValueTypes::Table)
            .and_where(Expr::col(ValueTypes::Id).is_in(1..=3))
            .to_owned();

        m.exec_stmt(stmt).await?;

        Ok(())
    }
}
