use super::{ArgumentEncoding, FromReq};
use crate::request::Req;
use serde::de::DeserializeOwned;

/// Passes URL-encoded arguments in the query string of a `GET` request.
pub struct GetUrl;

impl ArgumentEncoding for GetUrl {
    const CONTENT_TYPE: &'static str = "application/x-www-form-urlencoded";

    type Error = serde_qs::Error;
}

impl<T, State, Request> FromReq<State, Request, GetUrl> for T
where
    T: DeserializeOwned,
    Request: Req<State>,
{
    fn from_req(req: Request) -> Result<Self, <GetUrl as ArgumentEncoding>::Error> {
        let url = req.as_url();
        let args = serde_qs::from_str::<Self>(url)?;
        Ok(args)
    }
}

/// Passes URL-encoded arguments in the body of a `POST` request.
pub struct PostUrl;

impl ArgumentEncoding for PostUrl {
    const CONTENT_TYPE: &'static str = "application/x-www-form-urlencoded";

    type Error = serde_qs::Error;
}

impl<T, State, Request> FromReq<State, Request, PostUrl> for T
where
    T: DeserializeOwned,
    Request: Req<State>,
{
    fn from_req(req: Request) -> Result<Self, <PostUrl as ArgumentEncoding>::Error> {
        let body = req.into_string();
        let args = serde_qs::from_str::<Self>(&body)?;
        Ok(args)
    }
}
