use sqlx::MySqlPool;

use crate::config::Config;

#[derive(Clone)]
pub struct Context {
    pub config: Config,
    pub pool: MySqlPool,
}
