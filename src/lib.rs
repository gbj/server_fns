#![feature(return_position_impl_trait_in_trait)]
#![feature(async_fn_in_trait)]

pub mod client;
pub mod codec;
pub mod error;
pub mod request;
pub mod response;

use client::Client;
use codec::{Encoding, FromReq, FromRes, IntoReq, IntoRes};
use error::ServerFnError;
use request::Req;
use response::Res;
use std::future::Future;

pub trait ServerFn
where
    Self: Send
        + Sync
        + FromReq<Self::ServerRequest, Self::InputEncoding>
        + IntoReq<<Self::Client as Client>::Request, Self::InputEncoding>,
{
    const PATH: &'static str;

    /// The type of the HTTP client that will send the request from the client side.
    ///
    /// For example, this might be `gloo-net` in the browser, or `reqwest` for a desktop app.
    type Client: Client;

    /// The type of the HTTP request when received by the server function on the server side.
    type ServerRequest: Req + Send + Sync;

    /// The type of the HTTP response returned by the server function on the server side.
    type ServerResponse: Res + Send + Sync;

    /// The return type of the server function.
    ///
    /// This needs to be converted into `ServerResponse` on the server side, and converted
    /// *from* `ClientResponse` when received by the client.
    type Output: IntoRes<Self::ServerResponse, Self::OutputEncoding>
        + FromRes<<Self::Client as Client>::Response, Self::OutputEncoding>
        + Send
        + Sync;

    type InputEncoding: Encoding;
    type OutputEncoding: Encoding;

    // the body of the fn
    fn run_body(self) -> Self::Output;

    fn run_on_server(
        req: Self::ServerRequest,
    ) -> impl Future<Output = Self::ServerResponse> + Send + Sync {
        async {
            Self::execute_on_server(req)
                .await
                .unwrap_or_else(Self::ServerResponse::error_response)
        }
    }

    fn run_on_client(
        self,
    ) -> impl Future<Output = Result<Self::Output, ServerFnError>> + Send + Sync {
        async move {
            let req = self.into_req(Self::PATH)?;
            let res = Self::Client::send(req).await?;
            let output = Self::Output::from_res(res).await?;
            Ok(output)
        }
    }

    #[doc(hidden)]
    fn execute_on_server(
        req: Self::ServerRequest,
    ) -> impl Future<Output = Result<Self::ServerResponse, ServerFnError>> + Send + Sync {
        async {
            let this = Self::from_req(req).await?;
            let output = this.run_body();
            let res = output.into_res().await?;
            Ok(res)
        }
    }
}
