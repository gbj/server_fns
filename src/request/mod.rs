mod request;
mod http_request;

use futures::Stream;
use serde::ser::Error;
use bytes::Bytes;

pub trait Req<State, Error>
    where State: Sized, Error: std::error::Error {
    fn as_url(&self) -> &str;

    fn try_into_string(self) -> Result<String, Error>;

    async fn try_into_bytes(self) -> Result<Bytes, Error>;

    fn try_into_stream(self) -> dyn Stream<Item=State>;
}
