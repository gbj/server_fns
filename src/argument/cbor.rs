use std::fmt::Display;

use super::{ArgumentEncoding, FromReq};
use crate::{error::ServerFnError, request::Req};
use async_trait::async_trait;
use serde::de::DeserializeOwned;

/// Pass argument as JSON in the body of a POST Request
pub struct PostCbor;

impl ArgumentEncoding for PostCbor {
    const CONTENT_TYPE: &'static str = "application/cbor";
}

#[async_trait]
impl<T, State, Request> FromReq<State, Request, PostCbor> for T
where
    T: DeserializeOwned,
    Request: Req<State> + Send + 'static,
    Request::Error: Display,
    ciborium::de::Error<Request::Body>: From<ciborium::de::Error<std::io::Error>> + Display,
{
    async fn from_req(req: Request) -> Result<Self, ServerFnError> {
        let bytes = req
            .try_into_bytes()
            .await
            .map_err(|e| ServerFnError::Args(e.to_string()))?;
        let data = ciborium::de::from_reader(bytes.as_ref())
            .map_err(|e| ServerFnError::Args(e.to_string()))?;

        Ok(data)
    }
}
