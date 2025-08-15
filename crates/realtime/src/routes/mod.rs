pub mod docs;
pub mod health;

use std::sync::Arc;

use axum::{Router, routing::get};

use crate::state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health", get(health::handler))
        .nest("/docs", docs::router())
}
