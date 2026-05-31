use crate::services::ua_parser::UAParser;
use async_trait::async_trait;
use axum::Router;
use loco_rs::{
    app::{AppContext, Initializer},
    Result,
};

pub struct UAParserInitializer;

#[async_trait]
impl Initializer for UAParserInitializer {
    fn name(&self) -> String {
        "docs".to_string()
    }

    /// Этот хук вызывается после сборки основных роутов
    async fn after_routes(&self, router: Router, ctx: &AppContext) -> Result<Router> {
        let ua_parser_service = UAParser::new();
        ctx.shared_store.insert(ua_parser_service);

        Ok(router)
    }
}
