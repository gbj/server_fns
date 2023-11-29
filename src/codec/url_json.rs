use std::fmt::Display;

use super::{Encoding, Codec};
use crate::{error::ServerFnError, request::Req};
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::response::Res;

/// Pass arguments and receive responses as JSON in the body of a POST Request
pub struct GetUrlJson;
pub struct PostUrlJson;

impl Encoding for GetUrlJson {
    const REQUEST_CONTENT_TYPE: &'static str = "application/x-www-form-urlencoded";
    const RESPONSE_CONTENT_TYPE: &'static str = "application/json";
}
#[async_trait]
impl<T, RequestState, ResponseState, Request, Response> Codec<RequestState, ResponseState, Request, Response, GetUrlJson> for T
    where
        T: DeserializeOwned + Serialize + Send,
        Request: Req<RequestState> + Send + 'static,
        Request::Error: Display,
        Response: Res<ResponseState> + Send + 'static,
        Response::Error: Display,
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
        let (_parts, body) = res.into_parts();
        let data = res.try_into_string()?;
        serde_json::from_str(&data).map_err(|e| ServerFnError::Deserialization(e.to_string()))
    }

    async fn into_res(self) -> Result<Response, ServerFnError> {
        // Need to catch and error here, or handle Errors at a higher level
        let data = serde_json::to_string(&self)?;
        let re = Response::new(status, headers, body );
        Ok(res)
    }
}
