use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    Database { inner: sqlx::Error },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        #[serde(rename = "camelCase")]
        struct Dto {
            message: String,
            status_code: u16,
        }

        let (status_code, message) = match self {
            Self::Database { inner } => {
                error!("failed to execute database query: {inner}");

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal server error has occurred.",
                )
            }
        };

        Json(Dto {
            message: message.to_owned(),
            status_code: status_code.as_u16(),
        })
        .into_response()
    }
}

impl From<sqlx::Error> for Error {
    fn from(inner: sqlx::Error) -> Self {
        Self::Database { inner }
    }
}
