use chrono::Utc;
use loco_rs::prelude::*;
use migration::Expr;

use crate::models::_entities::refresh_tokens;

pub struct RefreshTokens;
#[async_trait]
impl Task for RefreshTokens {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "refresh_tokens".to_string(),
            detail: "Clear refresh tokens".to_string(),
        }
    }
    async fn run(&self, ctx: &AppContext, _vars: &task::Vars) -> Result<()> {
        println!("Task RefreshTokens started");
        let now = Utc::now();

        let result = refresh_tokens::Entity::update_many()
            .col_expr(refresh_tokens::Column::Revoked, Expr::value(true))
            .filter(refresh_tokens::Column::ExpiredAt.lt(now))
            .filter(refresh_tokens::Column::Revoked.eq(false))
            .exec(&ctx.db)
            .await?;

        tracing::info!(
            rows = result.rows_affected,
            "Revoked expired refresh tokens"
        );
        Ok(())
    }
}
