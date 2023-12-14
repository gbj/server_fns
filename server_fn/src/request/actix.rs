use crate::{error::ServerFnError, request::Req};
use actix_web::{FromRequest, HttpRequest};
use bytes::Bytes;
use send_wrapper::SendWrapper;
use std::future::Future;

pub struct ActixRequest(pub(crate) SendWrapper<HttpRequest>);

impl Req for ActixRequest {
    fn as_query(&self) -> Option<&str> {
        self.0.uri().query()
    }

    fn try_into_bytes(self) -> impl Future<Output = Result<Bytes, ServerFnError>> + Send {
        // Actix is going to keep this on a single thread anyway so it's fine to wrap it
        // with SendWrapper, which makes it `Send` but will panic if it moves to another thread
        SendWrapper::new(async move {
            Bytes::extract(&self.0)
                .await
                .map_err(|e| ServerFnError::Deserialization(e.to_string()))
        })
    }

    fn try_into_string(self) -> impl Future<Output = Result<String, ServerFnError>> + Send {
        // Actix is going to keep this on a single thread anyway so it's fine to wrap it
        // with SendWrapper, which makes it `Send` but will panic if it moves to another thread
        SendWrapper::new(async move {
            String::extract(&self.0)
                .await
                .map_err(|e| ServerFnError::Deserialization(e.to_string()))
        })
    }
}
