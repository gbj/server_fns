use std::fmt::Display;

use super::{Codec, Encoding};
use crate::error::{IntoErrorResponse, ServerFnError};
use async_trait::async_trait;
use axum::body::{Body, HttpBody};
use http_body_util::BodyExt;
use rkyv::{
    de::deserializers::SharedDeserializeMap, ser::serializers::AllocSerializer,
    validation::validators::DefaultValidator, Archive, CheckBytes, Deserialize, Serialize,
};
pub struct PostRkyv;

impl Encoding for PostRkyv {
    const REQUEST_CONTENT_TYPE: &'static str = "application/rkyv";
    const RESPONSE_CONTENT_TYPE: &'static str = "application/rkyv";
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
        PostRkyv,
    > for T
where
    T: Serialize<AllocSerializer<1024>> + Send,
    T: Archive,
    T::Archived: for<'a> CheckBytes<DefaultValidator<'a>> + Deserialize<T, SharedDeserializeMap>,
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

        rkyv::from_bytes::<T>(&body_bytes.as_ref()).map_err(|e| ServerFnError::Args(e.to_string()))
    }

    async fn into_req(self) -> Result<http::Request<Body>, ServerFnError> {
        let bytes = rkyv::to_bytes::<T, 1024>(&self)?.into_vec();
        let req = http::Request::builder()
            .method("POST")
            .header(
                http::header::CONTENT_TYPE,
                <PostRkyv as Encoding>::REQUEST_CONTENT_TYPE,
            )
            .body(Body::from(bytes))?;
        Ok(req)
    }
    async fn from_res(res: http::Response<ResponseBody>) -> Result<Self, ServerFnError> {
        let (_parts, body) = res.into_parts();

        let body_bytes = body
            .collect()
            .await
            .map(|c| c.to_bytes())
            .map_err(|e| ServerFnError::Deserialization(e.to_string()))?;
        rkyv::from_bytes::<T>(&body_bytes.as_ref())
            .map_err(|e| ServerFnError::Deserialization(e.to_string()))
    }

    async fn into_res(self) -> http::Response<Body> {
        let bytes = match rkyv::to_bytes::<T, 1024>(&self) {
            Ok(b) => b.into_vec(),
            Err(e) => return e.into_err_res(),
        };
        let res = http::Response::builder()
            .status(200)
            .header(
                http::header::CONTENT_TYPE,
                <PostRkyv as Encoding>::REQUEST_CONTENT_TYPE,
            )
            .body(Body::from(bytes))
            .unwrap();
        res
    }
}
