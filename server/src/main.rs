mod config;
mod context;
mod handlers;

#[allow(unused_imports)]
#[macro_use(debug, info, warn, error)]
extern crate log;

use anyhow::Result;
use axum::Router;
use tokio::net::TcpListener;

use crate::context::Context;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let ctx = Context::new().await?;
    let router = Router::new().with_state(ctx.clone());

    let addr = format!("{}:{}", ctx.config.web.host, ctx.config.web.port);
    info!("listening on {addr}");

    axum::serve(TcpListener::bind(addr).await?, router).await?;

    Ok(())
}
