use super::Res;
use crate::error::ServerFnError;
use actix_web::{http::header, http::StatusCode, HttpResponse};
use bytes::Bytes;
use send_wrapper::SendWrapper;

pub struct ActixResponse(pub(crate) SendWrapper<HttpResponse>);

impl Res for ActixResponse {
    fn try_from_string(content_type: &str, data: String) -> Result<Self, ServerFnError> {
        let mut builder = HttpResponse::build(StatusCode::OK);
        Ok(ActixResponse(SendWrapper::new(
            builder
                .insert_header((header::CONTENT_TYPE, content_type))
                .body(data),
        )))
    }

    fn try_from_bytes(content_type: &str, data: Bytes) -> Result<Self, ServerFnError> {
        let mut builder = HttpResponse::build(StatusCode::OK);
        Ok(ActixResponse(SendWrapper::new(
            builder
                .insert_header((header::CONTENT_TYPE, content_type))
                .body(data),
        )))
    }

    fn error_response(err: ServerFnError) -> Self {
        ActixResponse(SendWrapper::new(
            HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(err.to_string()),
        ))
    }
}