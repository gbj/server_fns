use crate::{error::ServerFnError, request::ClientReq, response::ClientRes};
use std::future::Future;

pub trait Client {
    type Request: ClientReq + Send;
    type Response: ClientRes + Send;

    fn send(
        req: Self::Request,
    ) -> impl Future<Output = Result<Self::Response, ServerFnError>> + Send;
}

#[cfg(feature = "browser")]
pub mod browser {
    use super::Client;
    use crate::{
        error::ServerFnError, request::browser::BrowserRequest, response::browser::BrowserResponse,
    };
    use send_wrapper::SendWrapper;
    use std::future::Future;

    pub struct BrowserClient;

    impl Client for BrowserClient {
        type Request = BrowserRequest;
        type Response = BrowserResponse;

        fn send(
            req: Self::Request,
        ) -> impl Future<Output = Result<Self::Response, ServerFnError>> + Send {
            SendWrapper::new(async move {
                req.0
                    .take()
                    .send()
                    .await
                    .map(|res| BrowserResponse(SendWrapper::new(res)))
                    .map_err(|e| ServerFnError::Request(e.to_string()))
            })
        }
    }
}

#[cfg(feature = "reqwest")]
pub mod reqwest {
    use super::Client;
    use crate::{error::ServerFnError, request::reqwest::CLIENT};
    use futures::TryFutureExt;
    use reqwest::{Request, Response};
    use std::future::Future;

    pub struct ReqwestClient;

    impl Client for ReqwestClient {
        type Request = Request;
        type Response = Response;

        fn send(
            req: Self::Request,
        ) -> impl Future<Output = Result<Self::Response, ServerFnError>> + Send {
            CLIENT
                .execute(req)
                .map_err(|e| ServerFnError::Request(e.to_string()))
        }
    }
}
