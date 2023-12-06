/* #[cfg(feature = "cbor")]
pub mod cbor;
#[cfg(feature = "json")]
pub mod json;
#[cfg(feature = "rkyv")]
pub mod rkyv; */
#[cfg(feature = "url_json")]
pub mod url_json;
use crate::{error::ServerFnError, request::Req, response::Res};
use async_trait::async_trait;
pub trait Encoding {
    const CONTENT_TYPE: &'static str;
}

#[async_trait]
pub trait FromReq<Request: Req, Encoding>
where
    Self: Sized,
{
    async fn from_req(req: Request) -> Result<Self, ServerFnError>;
}

#[async_trait]
pub trait IntoReq<Request: Req, Encoding> {
    async fn into_req(self) -> Result<Request, ServerFnError>;
}

#[async_trait]
pub trait FromRes<Response: Res, Encoding>
where
    Self: Sized,
{
    async fn from_res(res: Response) -> Result<Self, ServerFnError>;
}

#[async_trait]
pub trait IntoRes<Response: Res, Encoding> {
    async fn into_res(self) -> Response;
}
