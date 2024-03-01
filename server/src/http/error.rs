use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    Json(JsonRejection),
    Database(sqlx::Error),
    PasswordHash(password_hash::Error),
    TokenGeneration,
    MissingAuthorizationHeader,
    InvalidCredentials,
    ConflictingCredentials,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        #[serde(rename = "camelCase")]
        struct Dto {
            message: String,
            status_code: u16,
        }

        let ise = (
            StatusCode::INTERNAL_SERVER_ERROR,
            "An internal server error has occurred.",
        );

        let (status_code, message) = match self {
            Self::Json(_) => (StatusCode::BAD_REQUEST, "Malformed request."),
            Self::Database(inner) => {
                error!("failed to execute database query: {inner}");
                ise
            }
            Self::PasswordHash(inner) => {
                error!("failed to parse hashed password: {inner}");
                ise
            }
            Self::TokenGeneration => {
                error!("failed to generate token for user");
                ise
            }
            Self::MissingAuthorizationHeader => (StatusCode::UNAUTHORIZED, "Unauthorized."),
            Self::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials."),
            Self::ConflictingCredentials => (StatusCode::CONFLICT, "Conflicting credentials."),
        };

        Json(Dto {
            message: message.to_owned(),
            status_code: status_code.as_u16(),
        })
        .into_response()
    }
}

impl From<JsonRejection> for Error {
    fn from(inner: JsonRejection) -> Self {
        Self::Json(inner)
    }
}

impl From<sqlx::Error> for Error {
    fn from(inner: sqlx::Error) -> Self {
        Self::Database(inner)
    }
}

impl From<password_hash::Error> for Error {
    fn from(inner: password_hash::Error) -> Self {
        Self::PasswordHash(inner)
    }
}
