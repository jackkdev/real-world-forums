use anyhow::Result;
use axum::{Extension, Router};
use sqlx::MySqlPool;
use tokio::net::TcpListener;

use self::context::Context;
use crate::config::Config;

pub mod context;
pub mod endpoints;
pub mod error;
pub mod extractors;

pub async fn run(config: Config) -> Result<()> {
    let pool = MySqlPool::connect(config.database.url.as_str()).await?;

    let context = Context {
        config: config.clone(),
        pool,
    };

    let router = Router::new()
        .merge(endpoints::router())
        .layer(Extension(context.clone()))
        .with_state(context);

    let addr = format!("{}:{}", config.web.host, config.web.port);
    axum::serve(TcpListener::bind(addr).await?, router).await?;

    Ok(())
}
