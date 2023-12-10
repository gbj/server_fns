use crate::error::ServerFnError;

use super::ClientRes;
use bytes::Bytes;
pub use gloo_net::http::Response;
use send_wrapper::SendWrapper;
use std::future::Future;

pub struct BrowserResponse(pub(crate) SendWrapper<Response>);

impl ClientRes for BrowserResponse {
    fn try_into_string(self) -> impl Future<Output = Result<String, ServerFnError>> + Send + Sync {
        // the browser won't send this async work between threads (because it's single-threaded)
        // so we can safely wrap this
        SendWrapper::new(async move {
            self.0
                .text()
                .await
                .map_err(|e| ServerFnError::Deserialization(e.to_string()))
        })
    }

    fn try_into_bytes(self) -> impl Future<Output = Result<Bytes, ServerFnError>> + Send + Sync {
        // the browser won't send this async work between threads (because it's single-threaded)
        // so we can safely wrap this
        SendWrapper::new(async move {
            self.0
                .binary()
                .await
                .map(Bytes::from)
                .map_err(|e| ServerFnError::Deserialization(e.to_string()))
        })
    }
}
