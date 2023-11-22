#[cfg(feature = "cbor")]
pub mod cbor;
#[cfg(feature = "request")]
pub mod http_request;
#[cfg(feature = "json")]
pub mod json;
#[cfg(feature = "request")]
pub mod request;
#[cfg(feature = "url")]
pub mod url;

use crate::request::Req;
use async_trait::async_trait;
pub trait ArgumentEncoding<ErrorBody> {
    const CONTENT_TYPE: &'static str;

    type Error;
}

pub trait BodyEncoding<ErrorBody> {
    type Error;
}
// Who needs an encoding when we have a request?
pub trait RequestEncoding {
    type Error;
}
#[async_trait]
pub trait FromReq<State, Request, StdErrorTrait, ErrorBody, Enc>
where
    Enc: ArgumentEncoding<ErrorBody>,
    Request: Req<State, StdErrorTrait, ErrorBody> + Send,
    Self: Sized,
    StdErrorTrait: std::error::Error,
{
    async fn from_req(req: Request) -> Result<Self, Enc::Error>;
}
