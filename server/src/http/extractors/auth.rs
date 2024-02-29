use axum::{
    async_trait,
    body::Bytes,
    extract::{FromRequest, FromRequestParts},
    http::{request::Parts, HeaderMap},
    response::Response,
    Extension, RequestPartsExt,
};

use crate::http::context::Context;

pub struct Auth {}

#[async_trait]
impl<S> FromRequestParts<S> for Auth
where
    Bytes: FromRequest<S>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let headers = HeaderMap::from_request_parts(parts, state)
            .await
            .map_err(|err| match err {})?;

        let Extension(ctx) = parts
            .extract_with_state::<Extension<Context>, S>(state)
            .await
            .map_err(|_| panic!("Context is not added as an extension."))?;

        let header = match headers.get("Authorization") {
            Some(header) => header,
            None => return Err(Rejection),
        };

        Ok(Self {})
    }
}
