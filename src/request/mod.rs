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

    fn as_url(&self) -> &str;

    fn as_request(self) -> Self;

    async fn try_into_string(self) -> Result<String, Self::Error>;

    async fn try_into_bytes(self) -> Result<Bytes, Self::Error>;

    fn try_into_stream(self) -> Pin<Box<dyn Stream<Item = State> + Send + Sync>>;

    fn from_str(str: &str) -> Result<Self, Self::Error>;
    fn from_bytes(bytes: Bytes) -> Result<Self, Self::Error>;
    async fn from_stream() -> Result<Self, Self::Error>;
}
