use std::fmt::Display;

use super::{Codec, Encoding};
use crate::response::Res;
use crate::{error::ServerFnError, request::Req};
use async_trait::async_trait;
use serde::de::DeserializeOwned;

/// Pass arguments and receive responses as JSON in the body of a POST Request
pub struct PostJson;

impl Encoding for PostJson {
    const REQUEST_CONTENT_TYPE: &'static str = "application/json";
    const RESPONSE_CONTENT_TYPE: &'static str = "application/json";
}
#[async_trait]
impl<T, RequestState, ResponseState, Request, Response>
    Codec<RequestState, ResponseState, Request, Response, PostCbor> for T
where
    T: DeserializeOwned,
    Request: Req<RequestState> + Send + 'static,
    Request::Error: Display,
    Response: Res<ResponseState> + Send + 'static,
    Response::Error: Display,
    ciborium::de::Error<Request::Body>: From<ciborium::de::Error<std::io::Error>> + Display,
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

    async fn into_req(self) -> Result<Request, ServerFnError> {
        todo!()
    }

    async fn from_res(res: Response) -> Result<Self, ServerFnError> {
        todo!()
    }

    async fn into_res(self) -> Result<Response, ServerFnError> {
        let data = serde_json::to_string(&self)?;
        Ok(Response::from_string(data))
    }
}
