use super::Res;
use axum::body::HttpBody;
use http::Response;
use std::string::FromUtf8Error;
use thiserror::Error;

impl<B: Sized + Send> Res for Response<B>
where
    B: HttpBody,
    B::Data: Send,
    B::Error: Send,
{
}
