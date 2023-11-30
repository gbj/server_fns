use std::{string::FromUtf8Error};

use crate::request::Req;
use async_trait::async_trait;
use http::{Request};
use axum::body::HttpBody;
use thiserror::Error;

#[async_trait]
impl<'req, B: Sized + Send + Sync> Req<B> for Request<B>
where
    B: HttpBody,
    B::Data: Send + Sync,
    B::Error: Send + Sync,
{}


#[derive(Error, Debug)]
pub enum RequestError<B: HttpBody> {
    #[error("Error converting request body to bytes.")]
    BodyToBytes(B::Error),
    #[error("Request body was not a valid UTF-8 string.")]
    Utf8(FromUtf8Error),
}
