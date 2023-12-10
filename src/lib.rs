#![feature(return_position_impl_trait_in_trait)]
#![feature(async_fn_in_trait)]

pub mod codec;
pub mod error;
pub mod request;
pub mod response;

use codec::{FromReq, FromRes, IntoReq, IntoRes};
use error::ServerFnError;
use request::Req;
use response::Res;
use std::future::Future;

trait ServerFn
where
    Self: Send
        + FromReq<Self::Request, Self::ArgumentEncoding>
        + IntoReq<Self::Client, Self::ArgumentEncoding>,
{
    /// The type of the HTTP client that will send the request from the client side.
    ///
    /// For example, this might be `gloo-net` in the browser, or `reqwest` for a desktop app.
    type Client: Client;

    /// The type of the HTTP request when received by the server function on the server side.
    type Request: Req + Send;

    /// The type of the HTTP response returned by the server function on the server side.
    type ServerResponse: Res + Send;

    /// The type of the HTTP response as received by the server function on the client side.
    type ClientResponse: Res;

    /// The return type of the server function.
    ///
    /// This needs to be converted into `ServerResponse` on the server side, and converted
    /// *from* `ClientResponse` when received by the client.
    type Output: IntoRes<Self::ServerResponse, Self::OutputEncoding>
        + FromRes<Self::ClientResponse, Self::OutputEncoding>
        + Send;

    type ArgumentEncoding;
    type OutputEncoding;

    // the body of the fn
    fn call_fn_server(self) -> Self::Output;

    fn execute(
        req: Self::Request,
    ) -> impl Future<Output = Result<Self::ServerResponse, ServerFnError>> + Send {
        async {
            let this = Self::from_req(req).await?;
            let output = this.call_fn_server();
            let res = output.into_res().await?;
            Ok(res)
        }
    }

    fn respond(req: Self::Request) -> impl Future<Output = Self::ServerResponse> + Send {
        async {
            Self::execute(req)
                .await
                .unwrap_or_else(Self::ServerResponse::error_response)
        }
    }

    async fn make_request(self) {}
}

trait Client: Req {
    type Response: Res;

    async fn send(self) -> Self::Response;
}
