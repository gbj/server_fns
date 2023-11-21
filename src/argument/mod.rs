#[cfg(feature = "url")]
pub mod url;
pub mod json;
pub mod cbor;

use crate::request::Req;
use axum::extract::FromRequestParts;
use serde::Deserialize;
pub trait ArgumentEncoding {
    const CONTENT_TYPE: &'static str;

    type Error;
}

pub trait BodyEncoding{

    type Error;
}

pub trait FromReq<State, Request, Enc>
where
    Enc: ArgumentEncoding,
    Request: Req<State>,
    Self: Sized,
{
    fn from_req(req: Request) -> Result<Self, Enc::Error>;
}
