use std::io;

use crate::error::ServerFnError;

use super::Res;
use axum::body::Body;
use bytes::Bytes;
use futures::{Stream, StreamExt};
use http::Response;

impl Res for Response<Body> {
    fn try_from_string(content_type: &str, data: String) -> Result<Self, ServerFnError> {
        let builder = http::Response::builder();
        builder
            .status(200)
            .header(http::header::CONTENT_TYPE, content_type)
            .body(Body::from(data))
            .map_err(|e| ServerFnError::Response(e.to_string()))
    }

    fn try_from_bytes(content_type: &str, data: Bytes) -> Result<Self, ServerFnError> {
        let builder = http::Response::builder();
        builder
            .status(200)
            .header(http::header::CONTENT_TYPE, content_type)
            .body(Body::from(data))
            .map_err(|e| ServerFnError::Response(e.to_string()))
    }

    fn try_from_stream(
        content_type: &str,
        data: impl Stream<Item = Bytes> + Send + 'static,
    ) -> Result<Self, ServerFnError> {
        let body = Body::from_stream(data.map(|n| Ok::<Bytes, io::Error>(n)));
        let builder = http::Response::builder();
        builder
            .status(200)
            .header(http::header::CONTENT_TYPE, content_type)
            .body(body)
            .map_err(|e| ServerFnError::Response(e.to_string()))
    }

    fn error_response(err: ServerFnError) -> Self {
        Response::builder()
            .status(http::StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(err.to_string()))
            .unwrap()
    }
}
