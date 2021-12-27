use axum::extract::Extension;
use slog::{Logger};

pub async fn get_products(Extension(log): Extension<Logger>) -> &'static str {
    //info!(log, "get products");

    "get products"
}