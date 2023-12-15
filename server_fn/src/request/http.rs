use std::fmt::Display;

use crate::{error::ServerFnError, request::Req};
use axum::body::{Bytes, HttpBody};
use http::Request;
use http_body_util::BodyExt;

impl<B> Req for Request<B>
where
    B: HttpBody + Send,
    B::Error: Display,
    B::Data: Send,
{
    fn as_query(&self) -> Option<&str> {
        self.uri().query()
    }

    async fn try_into_bytes(self) -> Result<Bytes, ServerFnError> {
        let (_parts, body) = self.into_parts();

        body.collect()
            .await
            .map(|c| c.to_bytes())
            .map_err(|e| ServerFnError::Deserialization(e.to_string()))
    }

    async fn try_into_string(self) -> Result<String, ServerFnError> {
        let bytes = self.try_into_bytes().await?;
        let body = String::from_utf8(bytes.to_vec())
            .map_err(|e| ServerFnError::Deserialization(e.to_string()));
        println!("{body:?}");
        body
    }
}
