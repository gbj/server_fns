use super::{ArgumentEncoding, FromReq};
use crate::{error::ServerFnError, request::Req};
use async_trait::async_trait;
use core::fmt::Display;
use serde::de::DeserializeOwned;

/// Pass argument as JSON in the body of a POST Request
pub struct PostJson;

impl ArgumentEncoding for PostJson {
    const CONTENT_TYPE: &'static str = "application/json";
}

#[async_trait]
impl<T, State, Request> FromReq<State, Request, PostJson> for T
where
    T: DeserializeOwned,
    Request: Req<State> + Send + 'static,
    Request::Error: Display,
{
    async fn from_req(req: Request) -> Result<Self, ServerFnError> {
        let string = req
            .try_into_string()
            .await
            .map_err(|e| ServerFnError::Args(e.to_string()))?;
        let args = serde_json::from_str::<Self>(&string)
            .map_err(|e| ServerFnError::Args(e.to_string()))?;
        Ok(args)
    }
}
