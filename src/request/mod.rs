#[cfg(feature = "request")]
pub mod http_request;
#[cfg(feature = "request")]
pub mod request;

use async_trait::async_trait;
use bytes::Bytes;
use futures::Stream;

#[async_trait]
pub trait Req<State, StdErrorTrait, ErrorBody>
where
    State: Sized,
    StdErrorTrait: std::error::Error,
{
    fn as_url(&self) -> &str;

    fn try_into_string(self) -> Result<String, StdErrorTrait>;

    async fn try_into_bytes(self) -> Result<Bytes, StdErrorTrait>;

    fn try_into_stream(self) -> dyn Stream<Item = State>;
}
