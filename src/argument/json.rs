use super::{ArgumentEncoding, FromReq};
use crate::request::Req;
use serde::de::DeserializeOwned;

/// Pass argument as JSON in the body of a POST Request
pub struct PostJson;

impl ArgumentEncoding<ReqBody> for PostJson {
    const CONTENT_TYPE: &'static str = "application/json";

    type Error = serde_json::Error;
}

impl<T, State, Request> FromReq<State, Request, PostJson> for T
where
    T: DeserializeOwned,
    Request: Req<State> + Send,
{
    fn from_req(req: Request) -> Result<Self, <PostJson as BodyEncoding>::Error> {
        let args = serde_json::from_str::<Self>(&req.into_string())?;
        Ok(args)
    }
}
