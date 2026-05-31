use crate::m20260404_225136_roles::Roles;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Query::insert()
            .into_table(Roles::Table)
            .columns([Roles::Id, Roles::Name])
            .values_panic([1.into(), "admin".into()])
            .to_owned();

        m.exec_stmt(stmt).await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Query::delete()
            .from_table(Roles::Table)
            .and_where(Expr::col(Roles::Id).eq(1))
            .to_owned();

        m.exec_stmt(stmt).await?;

        Ok(())
    }
}
