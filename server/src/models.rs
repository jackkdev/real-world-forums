use chrono::serde::ts_nanoseconds;
use serde::{Deserialize, Serialize};
use sqlx::{
    prelude::FromRow,
    types::chrono::{DateTime, Utc},
};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i32,

    pub username: String,

    pub password: String,

    #[serde(with = "ts_nanoseconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_nanoseconds")]
    pub updated_at: DateTime<Utc>,
}
