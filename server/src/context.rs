use anyhow::Result;
use sqlx::MySqlPool;

use crate::config::Config;

#[derive(Clone)]
pub struct Context {
    pub config: Config,
    pub pool: MySqlPool,
}

impl Context {
    pub async fn new() -> Result<Self> {
        let config = Config::new()?;
        let pool = MySqlPool::connect(config.database.url.as_str()).await?;

        info!("connected to the database");

        Ok(Self { config, pool })
    }
}
