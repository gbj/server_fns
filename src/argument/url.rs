use super::{ArgumentEncoding, FromReq};
use crate::request::Req;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
/// Passes URL-encoded arguments in the query string of a `GET` request.
pub struct GetUrl;

impl<ErrorBody> ArgumentEncoding<ErrorBody> for GetUrl {
    const CONTENT_TYPE: &'static str = "application/x-www-form-urlencoded";

    type Error = serde_qs::Error;
}
#[async_trait]
impl<T, State, Request> FromReq<State, Request, GetUrl> for T
where
    T: DeserializeOwned,
    Request: Req<State> + Send + 'static,
    serde_qs::Error: From<<Request as Req<State>>::Error>

{
    async fn from_req(
        req: Request,
    ) -> Result<Self, <GetUrl as ArgumentEncoding<Request::Body>>::Error> {
        let url = req.as_url();
        let args = serde_qs::from_str::<Self>(url)?;
        Ok(args)
    }
}

/// Passes URL-encoded arguments in the body of a `POST` request.
pub struct PostUrl;

impl<ErrorBody> ArgumentEncoding<ErrorBody> for PostUrl {
    const CONTENT_TYPE: &'static str = "application/x-www-form-urlencoded";

    type Error = serde_qs::Error;
}
#[async_trait]
impl<T, State, Request> FromReq<State, Request, PostUrl> for T
where
    T: DeserializeOwned,
    Request: Req<State> + Send + 'static,
    serde_qs::Error: From<<Request as Req<State>>::Error>

{
    async fn from_req(
        req: Request,
    ) -> Result<Self, <PostUrl as ArgumentEncoding<Request::Body>>::Error> {
        let body = req.try_into_string().await?;
        let args = serde_qs::from_str::<Self>(&body)?;
        Ok(args)
    }
}
