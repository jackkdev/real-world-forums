use anyhow::Result;
use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Config {
    pub web: WebConfig,
    pub database: DatabaseConfig,
}

impl Config {
    pub fn new() -> Result<Self> {
        Ok(config::Config::builder()
            .add_source(config::Environment::with_prefix("FORUMS"))
            .add_source(config::File::with_name("App"))
            .build()?
            .try_deserialize()?)
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct WebConfig {
    pub host: String,
    pub port: u16,
    pub secret: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}
