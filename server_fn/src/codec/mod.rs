#[cfg(feature = "cbor")]
mod cbor;
#[cfg(feature = "cbor")]
pub use cbor::*;
#[cfg(feature = "json")]
mod json;
#[cfg(feature = "json")]
pub use json::*;
#[cfg(feature = "rkyv")]
mod rkyv;
#[cfg(feature = "rkyv")]
pub use rkyv::*;
#[cfg(feature = "url")]
mod url;
use crate::error::ServerFnError;
use futures::Future;
#[cfg(feature = "url")]
pub use url::*;

mod stream;
pub use stream::*;

pub trait FromReq<Request, Encoding>
where
    Self: Sized,
{
    fn from_req(req: Request) -> impl Future<Output = Result<Self, ServerFnError>> + Send;
}

pub trait IntoReq<Request, Encoding> {
    fn into_req(self, path: &str) -> Result<Request, ServerFnError>;
}

pub trait FromRes<Response, Encoding>
where
    Self: Sized,
{
    fn from_res(res: Response) -> impl Future<Output = Result<Self, ServerFnError>> + Send;
}

pub trait IntoRes<Response, Encoding> {
    fn into_res(self) -> impl Future<Output = Result<Response, ServerFnError>> + Send;
}

pub trait Encoding {
    const CONTENT_TYPE: &'static str;
}
