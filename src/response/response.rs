use super::Res;
use axum::body::HttpBody;
use http::Response;
use std::string::FromUtf8Error;
use thiserror::Error;

impl<B: Sized + Send + Sync> Res<B> for Response<B>
where
    B: HttpBody,
    B::Data: Send + Sync,
    B::Error: Send + Sync,
{
}

#[derive(Error, Debug)]
pub enum ResponseError<B: HttpBody> {
    #[error("Error converting request body to bytes.")]
    BodyToBytes(B::Error),
    #[error("Response body was not a valid UTF-8 string.")]
    Utf8(FromUtf8Error),
}
