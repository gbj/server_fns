mod request;
mod http_request;

use futures::Stream;
use http::Request;

pub trait Req<State>
    where State: Sized {
    fn as_url(&self) -> &str;

    fn into_string(self) -> String;

    fn into_bytes(self) -> Vec<u8>;

    fn into_stream(self) -> dyn Stream<Item=State>;
}
