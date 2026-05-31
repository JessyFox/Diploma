use crate::m20260404_225817_rights::Rights;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Query::insert()
            .into_table(Rights::Table)
            .columns([Rights::Id, Rights::Name])
            .values_panic([1.into(), "view".into()])
            .values_panic([2.into(), "create".into()])
            .values_panic([3.into(), "update".into()])
            .values_panic([4.into(), "delete".into()])
            .values_panic([5.into(), "grant".into()])
            .to_owned();

        m.exec_stmt(stmt).await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Query::delete()
            .from_table(Rights::Table)
            .and_where(Expr::col(Rights::Id).is_in(1..=5))
            .to_owned();

        m.exec_stmt(stmt).await?;

        Ok(())
    }
}
