#[cfg(feature = "actix")]
pub mod actix;
#[cfg(feature = "browser")]
pub mod browser;
#[cfg(feature = "axum")]
pub mod http;

use crate::error::ServerFnError;
use bytes::Bytes;
use std::future::Future;

/// Represents the response as created by the server;
pub trait Res: Sized {
    /// Attempts to convert a UTF-8 string into an HTTP response.
    fn try_from_string(content_type: &str, data: String) -> Result<Self, ServerFnError>;

    /// Attempts to convert a binary blob represented as Bytes into an HTTP response.
    fn try_from_bytes(content_type: &str, data: Bytes) -> Result<Self, ServerFnError>;

    fn error_response(err: ServerFnError) -> Self;
}

/// Represents the response as received by the client.
pub trait ClientRes {
    /// Attempts to extract a UTF-8 string from an HTTP response.
    fn try_into_string(self) -> impl Future<Output = Result<String, ServerFnError>> + Send;

    /// Attempts to extract a binary blob from an HTTP response.
    fn try_into_bytes(self) -> impl Future<Output = Result<Bytes, ServerFnError>> + Send;
}

/// A mocked response type that can be used in place of the actual server response,
/// when compiling for the browser.
pub struct BrowserMockRes;

impl Res for BrowserMockRes {
    fn try_from_string(content_type: &str, data: String) -> Result<Self, ServerFnError> {
        unreachable!()
    }

    fn try_from_bytes(content_type: &str, data: Bytes) -> Result<Self, ServerFnError> {
        unreachable!()
    }

    fn error_response(err: ServerFnError) -> Self {
        unreachable!()
    }
}
