mod config;
mod http;
mod models;

#[allow(unused_imports)]
#[macro_use(debug, info, warn, error)]
extern crate log;

use anyhow::Result;

use crate::config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    http::run(Config::new()?).await?;

    Ok(())
}
