use axum::{extract::State, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

use crate::http::{context::Context, error::Result};

pub fn router() -> Router<Context> {
    Router::new()
}
