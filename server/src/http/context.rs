use sqlx::MySqlPool;

use super::token_manager::TokenManager;
use crate::config::Config;

#[derive(Clone)]
pub struct Context {
    pub config: Config,
    pub pool: MySqlPool,
    pub token_manager: TokenManager,
}
