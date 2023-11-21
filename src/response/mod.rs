#[cfg(feature = "request")]
pub mod response;
#[cfg(feature = "request")]
pub mod http_response;

use futures::Stream;

pub trait Res {
    fn from_string(string: String) -> Self;

    fn from_bytes(bytes: Vec<u8>) -> Self;

    fn from_stream<T>(stream: impl Stream<Item = T>) -> Self;
}
