#[cfg(feature = "url")]
pub mod url;

use crate::request::Req;

pub trait ArgumentEncoding {
    const CONTENT_TYPE: &'static str;

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
