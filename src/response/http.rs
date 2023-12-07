use crate::error::ServerFnError;

use super::Res;
use axum::body::Body;
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

    fn try_from_bytes(content_type: &str, data: Vec<u8>) -> Result<Self, ServerFnError> {
        let builder = http::Response::builder();
        builder
            .status(200)
            .header(http::header::CONTENT_TYPE, content_type)
            .body(Body::from(data))
            .map_err(|e| ServerFnError::Response(e.to_string()))
    }

    fn error_response(err: ServerFnError) -> Self {
        Response::builder()
            .status(http::StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(err.to_string()))
            .unwrap()
    }
}
