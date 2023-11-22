use std::pin::Pin;
use std::string::FromUtf8Error;
use async_trait::async_trait;
use http::Response;
use super::Res;
use bytes::Bytes;
use thiserror::Error;
use futures::Stream;
use hyper::body::HttpBody;

#[async_trait]
impl<B: Sized + Send + Sync> Res<B> for Response<B>
    where
        B: HttpBody,
        B::Data: Send + Sync,
        B::Error: Send + Sync,
{
    type Body = B;
    type Error = ResponseError<B>;

    async fn try_into_string(self) -> Result<String, Self::Error> {
        let body_bytes = hyper::body::to_bytes(self.into_body())
            .await
            .map_err(ResponseError::BodyToBytes)?;
        Ok(String::from_utf8(body_bytes.to_vec()).map_err(ResponseError::Utf8)?)
    }

    async fn try_into_bytes(self) -> Result<Bytes, Self::Error> {
        todo!()
    }

    async fn try_into_stream<T>(self) -> Pin<Box<dyn Stream<Item=B> + Send + Sync>> {
        todo!()
    }
}

#[derive(Error, Debug)]
pub enum ResponseError<B: HttpBody> {
    #[error("Error converting request body to bytes.")]
    BodyToBytes(B::Error),
    #[error("Response body was not a valid UTF-8 string.")]
    Utf8(FromUtf8Error),
}
