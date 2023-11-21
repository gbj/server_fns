pub mod argument;
pub mod output;
pub mod request;
pub mod response;

use async_trait::async_trait;
use argument::ArgumentEncoding;
use output::{IntoRes, OutputEncoding};
use request::Req;
use response::Res;

use crate::argument::FromReq;

#[async_trait]
trait ServerFn<State, Request, Response, StdErrorTrait, ErrorBody>
where
    Response: Res,
    Request: Req<State, StdErrorTrait, ErrorBody> + Send + 'static,
    StdErrorTrait: std::error::Error,
    Self: FromReq<State, Request, StdErrorTrait, ErrorBody, Self::ArgumentEnc>,
{
    type Request;
    type ArgumentEnc: ArgumentEncoding<ErrorBody>;
    type ResponseEnc: OutputEncoding<ErrorBody>;
    type Output: IntoRes<Self::ResponseEnc, Response, ErrorBody>;

    // the body of the fn
    fn call_fn_server(self) -> Self::Output;

    async fn respond_to_request(req: Request) -> Result<Response, ()> {
        let this = Self::from_req(req).await.map_err(|_| ())?; // TODO handle errors properly
        let output = this.call_fn_server();
        let res = output.into_res().map_err(|_| ())?; // TODO errors
        Ok(res)
    }
}
