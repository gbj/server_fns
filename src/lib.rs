pub mod error;
pub mod codec;
pub mod request;
pub mod response;

use async_trait::async_trait;
use core::fmt::Display;
use error::ServerFnError;
use request::Req;
use response::Res;

use crate::codec::{Codec, Encoding};

#[async_trait]
trait ServerFn<RequestState, ResponseState, Request, Response>
where
    Response: Res<ResponseState> + Send + 'static,
    Request: Req<RequestState> + Send + 'static,
    Request::Error: Display,
    Self: Codec<RequestState, ResponseState, Request, Response, Self::Encoding>,
    <Response as Res<ResponseState>>::Error: Display,

{
    type Request;
    type Response;
    type Encoding: Encoding;
    type Output: Codec<RequestState, ResponseState, Request, Response, Self::Encoding>;

    // the body of the fn
    fn call_fn_server(self) -> Self::Output;

    async fn respond_to_request(req: Request) -> Result<Response, ServerFnError> {
        let this = Self::from_req(req).await?;
        let output = this.call_fn_server();
        let res = output
            .into_res()
            .await
            .map_err(|e| ServerFnError::Response(e.to_string()))?;
        Ok(res)
    }
}
