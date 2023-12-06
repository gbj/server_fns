use async_trait::async_trait;
use bytes::Bytes;

use crate::error::ServerFnError;

#[cfg(feature = "actix")]
pub mod http_request;
#[cfg(feature = "axum")]
pub mod request;

#[async_trait]
pub trait Req {
    fn as_url(&self) -> &str;

    async fn try_into_string(self) -> Result<String, ServerFnError>;

    async fn try_into_bytes(self) -> Result<Bytes, ServerFnError>;
}

#[async_trait]
pub trait ReqFromString: Sized {
    async fn try_from_string(
        method: &str,
        content_type: &str,
        data: String,
    ) -> Result<Self, ServerFnError>;
}
