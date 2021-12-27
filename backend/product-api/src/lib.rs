use axum::{routing::get, AddExtensionLayer, Router};
use middleware::policy_enforcer::PolicyEnforcerLayer;
use slog::Logger;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer, ServiceBuilderExt};

pub mod config;
mod handlers;
pub mod logs;
pub mod middleware;

pub fn app(log: Logger) -> Router {
    let middleware = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .compression()
        .layer(CorsLayer::permissive())
        .layer(AddExtensionLayer::new(log.clone()))
        .layer(PolicyEnforcerLayer::new(log.clone()))
        .into_inner();

    Router::new()
        .route("/api/products", get(handlers::get_products))
        .layer(middleware)
}
