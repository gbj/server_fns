#[cfg(feature = "request")]
pub mod http_request;
#[cfg(feature = "request")]
pub mod request;

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

    async fn try_into_string(self) -> Result<String, Self::Error>;

    async fn try_into_bytes(self) -> Result<Bytes, Self::Error>;

    fn try_into_stream(self) -> dyn Stream<Item = State>;
}
