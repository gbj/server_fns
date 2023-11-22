#[cfg(feature = "cbor")]
pub mod cbor;
#[cfg(feature = "request")]
pub mod http_request;
//#[cfg(feature = "serde_json")]
//pub mod json;
#[cfg(feature = "request")]
pub mod request;
#[cfg(feature = "url")]
pub mod url;

use core::fmt::Display;

use crate::{error::ServerFnError, request::Req};
use async_trait::async_trait;

pub trait ArgumentEncoding {
    const CONTENT_TYPE: &'static str;
}

#[async_trait]
pub trait FromReq<State, Request, Enc>
where
    Enc: ArgumentEncoding,
    Request: Req<State> + Send,
    Request::Error: Display,
    Self: Sized,
{
    async fn from_req(req: Request) -> Result<Self, ServerFnError>;
}
