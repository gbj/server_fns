use super::{BodyEncoding, FromReq};
use crate::request::Req;
use serde::de::DeserializeOwned;

/// Pass argument as JSON in the body of a POST Request
pub struct PostJson;

impl BodyEncoding for PostJson {
    type Error = impl std::error::Error;
}

impl<T, State, Request> FromReq<State, Request, PostJson> for T
where
    T: DeserializeOwned,
    Request: Req<State>,
{
    fn from_req(req: Request) -> Result<Self, <PostJson as BodyEncoding>::Error> {
        let args = serde_json::from_str::<Self>(&req.into_string())?;
        Ok(args)
    }
}
