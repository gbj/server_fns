use std::fmt::Display;

use super::{Codec, Encoding};
use crate::error::ServerFnError;
use async_trait::async_trait;
use axum::body::{Body, HttpBody};
use http_body_util::BodyExt;
use serde::de::DeserializeOwned;
use serde::Serialize;
/// Pass argument as JSON in the body of a POST Request
pub struct PostCbor;

impl Encoding for PostCbor {
    const REQUEST_CONTENT_TYPE: &'static str = "application/cbor";
    const RESPONSE_CONTENT_TYPE: &'static str = "application/cbor";
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
        PostCbor,
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
        let data = ciborium::de::from_reader(body_bytes.as_ref())
            .map_err(|e| ServerFnError::Args(e.to_string()))?;
        Ok(data)
    }

    async fn into_req(self) -> Result<http::Request<Body>, ServerFnError> {
        let mut buffer: Vec<u8> = Vec::new();
        ciborium::ser::into_writer(&self, &mut buffer)?;
        let req = http::Request::builder()
            .method("POST")
            .header(
                http::header::CONTENT_TYPE,
                <PostCbor as Encoding>::REQUEST_CONTENT_TYPE,
            )
            .body(Body::from(buffer))?;
        Ok(req)
    }
    async fn from_res(res: http::Response<ResponseBody>) -> Result<Self, ServerFnError> {
        let (_parts, body) = res.into_parts();

        let body_bytes = body
            .collect()
            .await
            .map(|c| c.to_bytes())
            .map_err(|e| ServerFnError::Deserialization(e.to_string()))?;

        ciborium::de::from_reader(body_bytes.as_ref())
            .map_err(|e| ServerFnError::Args(e.to_string()))
    }

    async fn into_res(self) -> Result<http::Response<Body>, ServerFnError> {
        let mut buffer: Vec<u8> = Vec::new();
        ciborium::ser::into_writer(&self, &mut buffer)?;

        let res = http::Response::builder()
            .status(200)
            .header(
                http::header::CONTENT_TYPE,
                <PostCbor as Encoding>::REQUEST_CONTENT_TYPE,
            )
            .body(Body::from(buffer))?;
        Ok(res)
    }
}
