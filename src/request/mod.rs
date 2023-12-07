use bytes::Bytes;
use std::future::Future;

use crate::error::ServerFnError;

#[cfg(feature = "actix")]
pub mod actix;
#[cfg(feature = "axum")]
pub mod http;

/// Represents a request as made by the client.
pub trait ClientReq: Sized {
    fn try_from_string(
        method: &str,
        content_type: &str,
        query: &str,
        body: String,
    ) -> impl Future<Output = Result<Self, ServerFnError>> + Send;

    fn try_from_bytes(
        method: &str,
        content_type: &str,
        query: &str,
        body: Vec<u8>,
    ) -> impl Future<Output = Result<Self, ServerFnError>> + Send;
}

/// Represents the request as received by the server.
pub trait Req: Sized {
    /// Returns the query string of the request’s URL, starting after the `?`.
    fn as_query(&self) -> Option<&str>;

    /// Attempts to extract the body of the request into [`Bytes`].
    fn try_into_bytes(self) -> impl Future<Output = Result<Bytes, ServerFnError>> + Send;

    /// Attempts to convert the body of the request into a string.
    fn try_into_string(self) -> impl Future<Output = Result<String, ServerFnError>> + Send;
}
