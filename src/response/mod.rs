#[cfg(feature = "request")]
pub mod http_response;
#[cfg(feature = "request")]
pub mod response;

use std::pin::Pin;
use async_trait::async_trait;
use bytes::Bytes;
use futures::Stream;

/// Since Req is about extracting, I'm assuming this one is too.
#[async_trait]
pub trait Res<State>
    where State: Sized
{
    type Body;
    type Error;
    type Parts;
    type Status;
    type HeaderMap;
    type Builder;
    fn new(status: Self::Status, headers: Self::HeaderMap, body: Self::Body) -> Self;
    fn body(&self) -> &Self::Body;
    fn into_body(self) -> Self::Body;
    fn into_parts(self) -> (Self::Parts,Self::Body);
    fn headers(&self) -> Self::HeaderMap;
    fn headers_mut(&mut self) -> &mut Self::HeaderMap;
    fn status(&self) -> &Self::Status;
    fn status_mut(&mut self) -> &mut Self::Status;
    async fn try_into_string(self) -> Result<String, Self::Error>;

    async fn try_into_bytes(self) -> Result<Bytes, Self::Error>;

    async fn try_into_stream<T>(self) -> Pin<Box<dyn Stream<Item = State> + Send + Sync>>;

}
