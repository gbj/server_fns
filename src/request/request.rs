use std::{fmt::Display, string::FromUtf8Error};

use crate::{error::ServerFnError, request::Req};
use async_trait::async_trait;
use axum::body::{Body, Bytes, HttpBody};
use http::Request;
use http_body_util::BodyExt;
use thiserror::Error;

#[async_trait]
impl<B> Req for Request<B>
where
    B: HttpBody + Send + Sync,
    B::Error: Display,
    B::Data: Send,
{
    fn as_url(&self) -> &str {
        todo!()
    }

    async fn try_into_string(self) -> Result<String, ServerFnError> {
        let bytes = self.try_into_bytes().await?;
        String::from_utf8(bytes.to_vec()).map_err(|e| ServerFnError::Deserialization(e.to_string()))
    }

    async fn try_into_bytes(self) -> Result<Bytes, ServerFnError> {
        let (_parts, body) = self.into_parts();

        body.collect()
            .await
            .map(|c| c.to_bytes())
            .map_err(|e| ServerFnError::Deserialization(e.to_string()))
    }
}
