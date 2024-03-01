pub mod context;
pub mod endpoints;
pub mod error;
pub mod extractors;
pub mod token_manager;

use anyhow::Result as BoxedResult;
use axum::{
    extract::{MatchedPath, Request},
    Extension, Router,
};
use sqlx::MySqlPool;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

pub use self::{
    context::Context,
    error::{Error, Result},
};
use crate::{config::Config, http::token_manager::TokenManager};

pub async fn run(config: Config) -> BoxedResult<()> {
    let pool = MySqlPool::connect(config.database.url.as_str()).await?;

    let context = Context {
        config: config.clone(),
        pool,
        token_manager: TokenManager::new(&config.web.secret)?,
    };

    let router = Router::new()
        .merge(endpoints::router())
        .layer(Extension(context.clone()))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|req: &Request| {
                    let method = req.method();
                    let uri = req.uri();

                    let matched_path = req
                        .extensions()
                        .get::<MatchedPath>()
                        .map(|matched_path| matched_path.as_str());

                    tracing::debug_span!("request", %method, %uri, matched_path)
                })
                .on_failure(()),
        )
        .with_state(context);

    let addr = format!("{}:{}", config.web.host, config.web.port);
    debug!("listening on {addr}");
    axum::serve(TcpListener::bind(addr).await?, router).await?;

    Ok(())
}
