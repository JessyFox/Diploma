use crate::docs::ApiDoc;
use async_trait::async_trait;
use axum::Router;
use loco_rs::{
    app::{AppContext, Initializer},
    Result,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub struct SwaggerUiInitializer;

#[async_trait]
impl Initializer for SwaggerUiInitializer {
    fn name(&self) -> String {
        "docs".to_string()
    }

    /// Этот хук вызывается после сборки основных роутов
    async fn after_routes(&self, router: Router, _ctx: &AppContext) -> Result<Router> {
        Ok(router.merge(SwaggerUi::new("/docs").url("/openapi.json", ApiDoc::openapi())))
    }
}
