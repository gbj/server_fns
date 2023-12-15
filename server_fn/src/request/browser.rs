use crate::error::ServerFnError;

use super::ClientReq;
use bytes::Bytes;
pub use gloo_net::http::Request;
use js_sys::Uint8Array;
use send_wrapper::SendWrapper;

pub struct BrowserRequest(pub(crate) SendWrapper<Request>);

impl ClientReq for BrowserRequest {
    fn try_new_get(path: &str, content_type: &str, query: &str) -> Result<Self, ServerFnError> {
        let mut url = path.to_owned();
        url.push('?');
        url.push_str(query);
        Ok(Self(SendWrapper::new(
            Request::get(&url)
                // TODO 'Accept' header
                .header("Content-Type", content_type)
                .build()
                .map_err(|e| ServerFnError::Request(e.to_string()))?,
        )))
    }

    fn try_new_post(path: &str, content_type: &str, body: String) -> Result<Self, ServerFnError> {
        Ok(Self(SendWrapper::new(
            Request::post(path)
                // TODO 'Accept' header
                .header("Content-Type", content_type)
                .body(body)
                .map_err(|e| ServerFnError::Request(e.to_string()))?,
        )))
    }

    fn try_new_post_bytes(
        path: &str,
        content_type: &str,
        body: Bytes,
    ) -> Result<Self, ServerFnError> {
        let body: &[u8] = &body;
        let body = Uint8Array::from(body).buffer();
        Ok(Self(SendWrapper::new(
            Request::post(path)
                // TODO 'Accept' header
                .header("Content-Type", content_type)
                .body(body)
                .map_err(|e| ServerFnError::Request(e.to_string()))?,
        )))
    }
}
