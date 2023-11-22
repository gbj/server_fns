pub mod argument;
pub mod error;
pub mod output;
pub mod request;
pub mod response;

use argument::ArgumentEncoding;
use async_trait::async_trait;
use core::fmt::Display;
use error::ServerFnError;
use output::{IntoRes, OutputEncoding};
use request::Req;
use response::Res;

use crate::argument::FromReq;

#[async_trait]
trait ServerFn<State, Request, Response>
where
    Response: Res,
    Request: Req<State> + Send + 'static,
    Request::Error: Display,
    Self: FromReq<State, Request, Self::ArgumentEnc>,
{
    type Request;
    type ArgumentEnc: ArgumentEncoding;
    type ResponseEnc: OutputEncoding;
    type Output: IntoRes<Self::ResponseEnc, Response>;

    // the body of the fn
    fn call_fn_server(self) -> Self::Output;

    async fn respond_to_request(req: Request) -> Result<Response, ServerFnError> {
        let this = Self::from_req(req).await?;
        let output = this.call_fn_server();
        let res = output
            .into_res()
            .map_err(|e| ServerFnError::Response(e.to_string()))?;
        Ok(res)
    }
}
