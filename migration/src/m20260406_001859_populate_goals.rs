use crate::m20260404_225800_goals::Goals;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Query::insert()
            .into_table(Goals::Table)
            .columns([Goals::Id, Goals::Name])
            .values_panic([1.into(), "stats".into()])
            .values_panic([2.into(), "dashboards".into()])
            .values_panic([3.into(), "users".into()])
            .values_panic([4.into(), "roles".into()])
            .values_panic([5.into(), "tables".into()])
            .values_panic([6.into(), "resources".into()])
            .values_panic([7.into(), "tasks".into()])
            .values_panic([8.into(), "nav_data".into()])
            .values_panic([9.into(), "user_pass".into()])
            .values_panic([10.into(), "admin".into()])
            .values_panic([11.into(), "reviews".into()])
            .to_owned();

        m.exec_stmt(stmt).await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Query::delete()
            .from_table(Goals::Table)
            .and_where(Expr::col(Goals::Id).is_in(1..=11))
            .to_owned();

        m.exec_stmt(stmt).await?;

        Ok(())
    }
}
