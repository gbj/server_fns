use bytes::Bytes;
use std::future::Future;

use crate::error::ServerFnError;

#[cfg(feature = "actix")]
pub mod actix;
#[cfg(feature = "browser")]
pub mod browser;
#[cfg(feature = "axum")]
pub mod http;

/// Represents a request as made by the client.
pub trait ClientReq: Sized {
    fn try_new_get(path: &str, content_type: &str, query: &str) -> Result<Self, ServerFnError>;

    fn try_new_post(path: &str, content_type: &str, body: String) -> Result<Self, ServerFnError>;

    fn try_new_post_bytes(
        path: &str,
        content_type: &str,
        body: Bytes,
    ) -> Result<Self, ServerFnError>;
}

/// Represents the request as received by the server.
pub trait Req: Sized {
    /// Returns the query string of the request’s URL, starting after the `?`.
    fn as_query(&self) -> Option<&str>;

    /// Attempts to extract the body of the request into [`Bytes`].
    fn try_into_bytes(self) -> impl Future<Output = Result<Bytes, ServerFnError>> + Send + Sync;

    /// Attempts to convert the body of the request into a string.
    fn try_into_string(self) -> impl Future<Output = Result<String, ServerFnError>> + Send + Sync;
}
