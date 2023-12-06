use super::{Encoding, FromReq, FromRes, IntoReq, Req};
use crate::error::{IntoErrorResponse, ServerFnError};
use crate::request::ReqFromString;
use crate::response::Res;
use async_trait::async_trait;
use axum::body::{Body, HttpBody};
use http_body_util::BodyExt;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Display;
/// Pass arguments and receive responses as JSON in the body of a POST Request
pub struct GetUrl;
pub struct PostUrl;

impl Encoding for GetUrl {
    const CONTENT_TYPE: &'static str = "application/x-www-form-urlencoded";
}

impl Encoding for PostUrl {
    const CONTENT_TYPE: &'static str = "application/x-www-form-urlencoded";
}

#[async_trait]
impl<T, Request> FromReq<Request, PostUrl> for T
where
    Request: Req + Send + 'static,
    T: DeserializeOwned,
{
    async fn from_req(req: Request) -> Result<Self, ServerFnError> {
        let string_data = req.try_into_string().await?;
        let args = serde_json::from_str::<Self>(&string_data)
            .map_err(|e| ServerFnError::Args(e.to_string()))?;
        Ok(args)
    }
}

#[async_trait]
impl<T, Request> IntoReq<Request, PostUrl> for T
where
    Request: Req + ReqFromString,
    T: Serialize + Send,
{
    async fn into_req(self) -> Result<Request, ServerFnError> {
        let qs = serde_qs::to_string(&self)?;
        Request::try_from_string("GET", GetUrl::CONTENT_TYPE, qs).await
    }
}

/* #[async_trait]
impl<T, Request, Response> Codec<Request, Response, GetUrlJson> for T
where
    T: DeserializeOwned + Serialize + Send,
    Request: Req + Send,
    Response: Res + Send,
{
    async fn from_req(req: Request) -> Result<Self, ServerFnError> {
        let string_data = req.try_into_string()?;

        let args = serde_json::from_str::<Self>(&string_data)
            .map_err(|e| ServerFnError::Args(e.to_string()))?;
        Ok(args)
    }

    async fn into_req(self) -> Result<Request, ServerFnError> {
        /* let qs = serde_qs::to_string(&self)?;
        let req = http::Request::builder()
            .method("GET")
            .header(
                http::header::CONTENT_TYPE,
                <GetUrlJson as Encoding>::REQUEST_CONTENT_TYPE,
            )
            .body(Body::from(qs))?;
        Ok(req) */
        todo!()
    }

    async fn from_res(res: Response) -> Result<Self, ServerFnError> {
        todo!()
        /* let (_parts, body) = res.into_parts();

        let body_bytes = body
            .collect()
            .await
            .map(|c| c.to_bytes())
            .map_err(|e| ServerFnError::Deserialization(e.to_string()))?;
        let string_data = String::from_utf8(body_bytes.to_vec())?;
        serde_json::from_str(&string_data)
            .map_err(|e| ServerFnError::Deserialization(e.to_string())) */
    }

    async fn into_res(self) -> Response {
        // Need to catch and err or here, or handle Errors at a higher level
        let data = match serde_json::to_string(&self) {
            Ok(d) => d,
            Err(e) => return e.into_err_res(),
        };
        let builder = http::Response::builder();
        let res = builder
            .status(200)
            .header(
                http::header::CONTENT_TYPE,
                <GetUrlJson as Encoding>::RESPONSE_CONTENT_TYPE,
            )
            .body(Body::from(data))
            .unwrap();
        res
    }
}
 */
