pub mod open;

use std::sync::Arc;

use axum::{Router, routing::get};

use crate::state::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/{id}", get(open::handler))
}
