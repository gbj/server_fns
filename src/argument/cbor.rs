use super::{BodyEncoding, FromReq};
use crate::request::Req;
use serde::de::DeserializeOwned;

/// Pass argument as JSON in the body of a POST Request
pub struct PostCbor;

impl BodyEncoding for PostCbor {
    // Currently annoyed that these error types take generics
    type Error = ciborium::de::Error<T>;
}

impl<T, State, Request> FromReq<State, Request, PostCbor> for T
    where
        T: DeserializeOwned,
        Request: Req<State>,
{
    fn from_req(req: Request) -> Result<Self, <PostCbor as BodyEncoding>::Error> {
        let data = ciborium::de::from_reader(&req.into_bytes())?;
        Ok(data)
    }
}
