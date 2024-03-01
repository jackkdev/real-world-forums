mod auth;
mod users;

use axum::Router;

use super::context::Context;

pub fn router() -> Router<Context> {
    Router::new().merge(users::router()).merge(auth::router())
}
