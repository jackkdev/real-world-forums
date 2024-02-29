use chrono::serde::ts_nanoseconds;
use serde::{Deserialize, Serialize};
use sqlx::{
    prelude::FromRow,
    types::chrono::{DateTime, Utc},
};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: u64,

    pub username: String,

    #[serde(with = "ts_nanoseconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_nanoseconds")]
    pub updated_at: DateTime<Utc>,
}
