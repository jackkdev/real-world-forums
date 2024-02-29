use axum::{extract::State, routing::get, Json, Router};
use serde::{Deserialize, Serialize};

use crate::http::{context::Context, error::Error};

pub fn router() -> Router<Context> {
    Router::new().route("/auth/login", get(login))
}

#[derive(Debug, Deserialize)]
struct LoginRequest {}

#[derive(Debug, Serialize)]
struct LoginResponse {}

async fn login(ctx: State<Context>) -> Result<Json<LoginResponse>, Error> {
    Ok(Json(LoginResponse {}))
}
