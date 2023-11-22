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

    async fn try_into_string(self) -> Result<String, Self::Error>;

    async fn try_into_bytes(self) -> Result<Bytes, Self::Error>;

    async fn try_into_stream<T>(self) -> Pin<Box<dyn Stream<Item = State> + Send + Sync>>;
}
