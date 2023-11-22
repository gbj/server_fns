use crate::request::Req;
use async_trait::async_trait;
use axum::body::Bytes;
use futures::Stream;
use http::Request;
use hyper::body::HttpBody;
use thiserror::Error;

#[async_trait]
impl<B: Sized> Req<B> for Request<B>
where
    B: HttpBody,
{
    type Body = B;
    type Error = RequestError<B>;

    fn as_url(&self) -> &str {
        todo!()
    }

    async fn try_into_string(self) -> Result<String, Self::Error> {
        let body_bytes = hyper::body::to_bytes(self.into_body()).await?;
        Ok(String::from_utf8(body_bytes.to_vec())?)
    }

    async fn try_into_bytes(self) -> Result<Bytes, Self::Error> {
        hyper::body::to_bytes(self.into_body()).await
    }

    fn try_into_stream(self) -> dyn Stream<Item = B> {
        todo!()
    }
}

#[derive(Error, Debug)]
pub enum RequestError<B: HttpBody> {
    #[error("Error converting request body to bytes: {0:?}")]
    BodyToBytes(B::Error),
}
