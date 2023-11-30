use std::fmt::Display;

use super::{Codec, Encoding};
use crate::{error::ServerFnError, request::Req};
use async_trait::async_trait;
use serde::de::DeserializeOwned;

/// Pass argument as JSON in the body of a POST Request
pub struct PostCbor;

impl Encoding for PostCbor {
    const REQUEST_CONTENT_TYPE: &'static str = "application/cbor";
    const RESPONSE_CONTENT_TYPE: &'static str = "application/cbor";
}

#[async_trait]
impl<T, RequestState, ResponseState, Request, Response>
    Codec<RequestState, ResponseState, Request, Response, PostCbor> for T
where
    T: DeserializeOwned,
    Request: Req<RequestState> + Send + 'static,
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

    async fn into_req(self) -> Result<Request, ServerFnError> {
        let mut buffer: Vec<u8> = Vec::new();
        ciborium::ser::into_writer(&self, &mut buffer)?;
        let mut req = Request::new();
        req.insert_header(http::header::CONTENT_TYPE);
    }
    async fn from_res(res: Response) -> Result<Self, ServerFnError> {
        todo!()
    }

    async fn into_res(self) -> Result<Response, ServerFnError> {
        let mut buffer: Vec<u8> = Vec::new();
        ciborium::ser::into_writer(&self, &mut buffer)?;

        let mut response = Response::new();

        Ok(Response::from_bytes(buffer))
    }
}
