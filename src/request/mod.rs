#[cfg(feature = "request")]
pub mod http_request;
#[cfg(feature = "request")]
pub mod request;

use std::pin::Pin;

use async_trait::async_trait;
use bytes::Bytes;
use futures::Stream;

#[async_trait]
pub trait Req<State>
where
    State: Sized,
{
    type Body;
    type Error;
    type Parts;
    type Uri;
    type Builder;
    type Method;
    type HeaderMap;

    fn into_parts(self) -> (Self::Parts,Self::Body);
    fn body(&self) -> &Self::Body;
    fn body_mut(&mut self) -> &mut Self::Body;
    fn into_body(self) -> Self::Body;
    fn headers(&self) -> Self::HeaderMap;
    fn headers_mut(&mut self) -> &mut Self::HeaderMap;
    fn uri(&self) -> &Self::Uri;
    fn uri_mut(&mut self) -> &mut Self::Uri;
    fn method(&self) -> &Self::Method;
    fn method_mut(&mut self) -> &mut Self::Method;
    fn builder() -> Self::Builder;
    async fn try_into_string(self) -> Result<String, Self::Error>;

    async fn try_into_bytes(self) -> Result<Bytes, Self::Error>;

    fn try_into_stream(self) -> Pin<Box<dyn Stream<Item = State> + Send + Sync>>;
}
