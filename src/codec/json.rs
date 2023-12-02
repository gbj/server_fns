use std::fmt::Display;

use super::{Codec, Encoding};
use crate::error::{IntoErrorResponse, ServerFnError};
use async_trait::async_trait;
use axum::body::{Body, HttpBody};
use http_body_util::BodyExt;
use serde::de::DeserializeOwned;
use serde::Serialize;
/// Pass arguments and receive responses as JSON in the body of a POST Request
pub struct PostJson;

impl Encoding for PostJson {
    const REQUEST_CONTENT_TYPE: &'static str = "application/json";
    const RESPONSE_CONTENT_TYPE: &'static str = "application/json";
}
#[async_trait]
impl<T, RequestBody, ResponseBody>
    Codec<
        RequestBody,
        ResponseBody,
        http::Request<RequestBody>,
        http::Response<ResponseBody>,
        Body,
        Body,
        http::Request<Body>,
        http::Response<Body>,
        PostJson,
    > for T
where
    T: DeserializeOwned + Serialize + Send,
    for<'a> RequestBody: HttpBody + Send + Sync + 'a,
    <RequestBody as HttpBody>::Error: Display + Send + Sync,
    <ResponseBody as HttpBody>::Error: Display + Send + Sync,
    for<'a> ResponseBody: HttpBody + Send + Sync + 'a,
    <ResponseBody as HttpBody>::Data: Send + Sync,
    <RequestBody as HttpBody>::Data: Send + Sync,
{
    async fn from_req(req: http::Request<RequestBody>) -> Result<Self, ServerFnError> {
        let (_parts, body) = req.into_parts();
        let body_bytes = body
            .collect()
            .await
            .map(|c| c.to_bytes())
            .map_err(|e| ServerFnError::Deserialization(e.to_string()))?;
        let string_data = String::from_utf8(body_bytes.to_vec())?;

        let args = serde_json::from_str::<Self>(&string_data)
            .map_err(|e| ServerFnError::Args(e.to_string()))?;
        Ok(args)
    }

    async fn into_req(self) -> Result<http::Request<Body>, ServerFnError> {
        let args = serde_json::to_string(&self)?;
        let req = http::Request::builder()
            .method("GET")
            .header(
                http::header::CONTENT_TYPE,
                <PostJson as Encoding>::REQUEST_CONTENT_TYPE,
            )
            .body(Body::from(args))?;
        Ok(req)
    }

    async fn from_res(res: http::Response<ResponseBody>) -> Result<Self, ServerFnError> {
        let (_parts, body) = res.into_parts();

        let body_bytes = body
            .collect()
            .await
            .map(|c| c.to_bytes())
            .map_err(|e| ServerFnError::Deserialization(e.to_string()))?;
        let string_data = String::from_utf8(body_bytes.to_vec())?;
        serde_json::from_str(&string_data)
            .map_err(|e| ServerFnError::Deserialization(e.to_string()))
    }

    async fn into_res(self) -> http::Response<Body> {
        // Need to catch and err or here, or handle Errors at a higher level
        let data = match serde_json::to_string(&self) {
            Ok(d) => d,
            Err(e) => return e.into_err_res(),
        };
        let builder = http::Response::builder();
        let res = match builder
            .status(200)
            .header(
                http::header::CONTENT_TYPE,
                <PostJson as Encoding>::RESPONSE_CONTENT_TYPE,
            )
            .body(Body::from(data))
        {
            Ok(r) => r,
            Err(e) => return e.into_err_res(),
        };
        res
    }
}
