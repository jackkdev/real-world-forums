use argon2::Argon2;
use axum::{extract::State, routing::post, Router};
use base64::prelude::*;
use chrono::Utc;
use password_hash::{PasswordHash, Salt};
use rand::RngCore;
use serde::{Deserialize, Serialize};

use crate::{
    http::{extractors::Json, Context, Error, Result},
    models::User,
};

/// Exports the router for this module.
pub fn router() -> Router<Context> {
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/register", post(register))
}

/// POST /auth/login
async fn login(
    State(ctx): State<Context>,
    Json(payload): Json<dto::LoginRequest>,
) -> Result<Json<dto::LoginResponse>> {
    // Attempt to find the user with the given username.
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT * FROM `users` 
        WHERE `username` = ?
    "#,
    )
    .bind(payload.username)
    .fetch_optional(&ctx.pool)
    .await?;

    let user = match user {
        Some(user) => user,
        None => return Err(Error::InvalidCredentials),
    };

    // Compare the input password with the hashed password.
    let hash = PasswordHash::new(user.password.as_str())?;

    match hash.verify_password(&[&Argon2::default()], payload.password.as_str()) {
        Ok(_) => {}
        Err(_) => return Err(Error::InvalidCredentials),
    };

    // Generate an authentication token.
    let token = ctx
        .token_manager
        .generate(user.id)
        .map_err(|_| Error::TokenGeneration)?;

    Ok(Json(dto::LoginResponse {
        token: token.as_str().to_owned(),
    }))
}

/// POST /auth/register
async fn register(
    State(ctx): State<Context>,
    Json(payload): Json<dto::RegisterRequest>,
) -> Result<Json<dto::RegisterResponse>> {
    // Attempt to find the user with the given username.
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT * FROM `users` 
        WHERE `username` = ?
    "#,
    )
    .bind(payload.username.as_str())
    .fetch_optional(&ctx.pool)
    .await?;

    // Return error if conflict exists.
    match user {
        Some(_) => return Err(Error::ConflictingCredentials),
        None => {}
    };

    // Generate the password hash.
    let mut salt = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut salt);
    let salt = BASE64_STANDARD.encode(&salt);

    let hash = PasswordHash::generate(Argon2::default(), payload.password, Salt::from_b64(&salt)?)?;

    // Create the new user record.
    let now = Utc::now();
    sqlx::query(
        r#"
        INSERT INTO `users` (`username`, `password`, `created_at`, `updated_at`)
        VALUES (?, ?, ?, ?)
    "#,
    )
    .bind(payload.username.as_str())
    .bind(hash.serialize().as_str())
    .bind(now.clone())
    .bind(now)
    .execute(&ctx.pool)
    .await?;

    Ok(Json(dto::RegisterResponse { ok: true }))
}

/// Data-transfer-objects for this module.
mod dto {
    use super::*;

    #[derive(Debug, Deserialize)]
    pub struct LoginRequest {
        pub username: String,
        pub password: String,
    }

    #[derive(Debug, Serialize)]
    pub struct LoginResponse {
        pub token: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct RegisterRequest {
        pub username: String,
        pub password: String,
    }

    #[derive(Debug, Serialize)]
    pub struct RegisterResponse {
        pub ok: bool,
    }
}
