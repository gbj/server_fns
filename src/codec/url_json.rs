use super::{Encoding, Codec};
use crate::{error::ServerFnError };
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use axum::body::HttpBody;
use std::fmt::Display;
use http_body_util::BodyExt;
/// Pass arguments and receive responses as JSON in the body of a POST Request
pub struct GetUrlJson;
pub struct PostUrlJson;

impl Encoding for GetUrlJson {
    const REQUEST_CONTENT_TYPE: &'static str = "application/x-www-form-urlencoded";
    const RESPONSE_CONTENT_TYPE: &'static str = "application/json";
}
#[async_trait]
impl<T, RequestBody, ResponseBody> Codec<RequestBody, ResponseBody, http::Request<RequestBody>, http::Response<ResponseBody>, GetUrlJson> for T
    where
        T: DeserializeOwned + Serialize + Send,
        RequestBody: HttpBody + Send + Sync,
        <RequestBody as HttpBody>::Error: Display + Send + Sync,
        <ResponseBody as HttpBody>::Error: Display + Send + Sync,
        ResponseBody: HttpBody + Send + Sync,
        <ResponseBody as HttpBody>::Data: Send + Sync,
        <RequestBody as HttpBody>::Data: Send + Sync,
{
    async fn from_req(req: http::Request<RequestBody>) -> Result<Self, ServerFnError> {

        let (_parts, body) = req.into_parts();
        let body_bytes = body.collect().await.map(|c| c.to_bytes()).map_err(|e| ServerFnError::Deserialization(e.to_string()))?;
        let string_data = String::from_utf8(body_bytes.to_vec())?;

        let args = serde_json::from_str::<Self>(&string_data)
            .map_err(|e| ServerFnError::Args(e.to_string()))?;
        Ok(args)
    }

    async fn into_req(self) -> Result<http::Request<RequestBody>, ServerFnError> {
        let qs = serde_qs::to_string(&self)?;
        let req = http::Request::builder().method("GET").header(http::header::CONTENT_TYPE, <GetUrlJson as Encoding>::REQUEST_CONTENT_TYPE).body(qs)?;
        Ok(req)
    }

    async fn from_res(res: http::Response<ResponseBody>) -> Result<Self, ServerFnError> {
        let (_parts, body) = res.into_parts();

        let body_bytes = body.collect().await.map(|c| c.to_bytes()).map_err(|e| ServerFnError::Deserialization(e.to_string()))?;
        let string_data = String::from_utf8(body_bytes.to_vec())?;
        serde_json::from_str(&string_data).map_err(|e| ServerFnError::Deserialization(e.to_string()))
    }

    async fn into_res(self) -> Result<http::Response<ResponseBody>, ServerFnError> {
        // Need to catch and err or here, or handle Errors at a higher level
        let data = serde_json::to_string(&self)?;
        let builder = http::Response::builder();
        let res = builder.status(200).header(http::header::CONTENT_TYPE, <GetUrlJson as Encoding>::RESPONSE_CONTENT_TYPE).body(data)?;
        Ok(res)
    }
}
