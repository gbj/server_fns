use super::{ArgumentEncoding, FromReq};
use crate::{error::ServerFnError, request::Req};
use async_trait::async_trait;
use core::fmt::Display;
use serde::de::DeserializeOwned;

/// Passes URL-encoded arguments in the query string of a `GET` request.
pub struct GetUrl;

impl ArgumentEncoding for GetUrl {
    const CONTENT_TYPE: &'static str = "application/x-www-form-urlencoded";
}

#[async_trait]
impl<T, State, Request> FromReq<State, Request, GetUrl> for T
where
    T: DeserializeOwned,
    Request: Req<State> + Send + 'static,
    Request::Error: Display,
{
    async fn from_req(req: Request) -> Result<Self, ServerFnError> {
        let url = req.as_url();
        let args =
            serde_qs::from_str::<Self>(url).map_err(|e| ServerFnError::Args(e.to_string()))?;
        Ok(args)
    }
}

/// Passes URL-encoded arguments in the body of a `POST` request.
pub struct PostUrl;

impl ArgumentEncoding for PostUrl {
    const CONTENT_TYPE: &'static str = "application/x-www-form-urlencoded";
}

#[async_trait]
impl<T, State, Request> FromReq<State, Request, PostUrl> for T
where
    T: DeserializeOwned,
    Request: Req<State> + Send + 'static,
    Request::Error: Display,
{
    async fn from_req(req: Request) -> Result<Self, ServerFnError> {
        let body = req
            .try_into_string()
            .await
            .map_err(|e| ServerFnError::Args(e.to_string()))?;
        let args =
            serde_qs::from_str::<Self>(&body).map_err(|e| ServerFnError::Args(e.to_string()))?;
        Ok(args)
    }
}
