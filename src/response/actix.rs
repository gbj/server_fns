use super::Res;
use crate::error::ServerFnError;
use actix_web::{body::BoxBody, http::header, http::StatusCode, HttpResponse};
use bytes::Bytes;

impl Res for HttpResponse<BoxBody> {
    fn try_from_string(content_type: &str, data: String) -> Result<Self, ServerFnError> {
        let mut builder = HttpResponse::build(StatusCode::OK);
        Ok(builder
            .insert_header((header::CONTENT_TYPE, content_type))
            .body(data))
    }

    fn try_from_bytes(content_type: &str, data: Bytes) -> Result<Self, ServerFnError> {
        let mut builder = HttpResponse::build(StatusCode::OK);
        Ok(builder
            .insert_header((header::CONTENT_TYPE, content_type))
            .body(data))
    }

    fn error_response(err: ServerFnError) -> Self {
        HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(err.to_string())
    }
}
