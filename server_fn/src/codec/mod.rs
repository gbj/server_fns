#[cfg(feature = "cbor")]
pub mod cbor;
#[cfg(feature = "json")]
pub mod json;
#[cfg(feature = "rkyv")]
pub mod rkyv;
#[cfg(feature = "url")]
pub mod url;
use crate::error::ServerFnError;
use futures::Future;

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
