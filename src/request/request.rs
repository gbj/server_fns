use std::{pin::Pin, string::FromUtf8Error};

use crate::request::Req;
use async_trait::async_trait;
use bytes::Bytes;
use futures::Stream;
use http::Request;
use hyper::body::HttpBody;
use thiserror::Error;

#[async_trait]
impl<B: Sized + Send + Sync> Req<B> for Request<B>
where
    B: HttpBody,
    B::Data: Send + Sync,
    B::Error: Send + Sync,
{
    type Body = B;
    type Error = RequestError<B>;

    fn as_request(self) -> Self {
        self
    }

    async fn try_into_string(self) -> Result<String, Self::Error> {
        let body_bytes = hyper::body::to_bytes(self.into_body())
            .await
            .map_err(RequestError::BodyToBytes)?;
        Ok(String::from_utf8(body_bytes.to_vec()).map_err(RequestError::Utf8)?)
    }

    async fn try_into_bytes(self) -> Result<Bytes, Self::Error> {
        hyper::body::to_bytes(self.into_body())
            .await
            .map_err(RequestError::BodyToBytes)
    }

    fn try_into_stream(self) -> Pin<Box<dyn Stream<Item = B> + Send + Sync>> {
        todo!()
    }
}

#[derive(Error, Debug)]
pub enum RequestError<B: HttpBody> {
    #[error("Error converting request body to bytes.")]
    BodyToBytes(B::Error),
    #[error("Request body was not a valid UTF-8 string.")]
    Utf8(FromUtf8Error),
}
