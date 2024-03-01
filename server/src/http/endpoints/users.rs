use axum::Router;

use crate::http::Context;

/// Exports the router for this module.
pub fn router() -> Router<Context> {
    Router::new()
}
