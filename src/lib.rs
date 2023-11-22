pub mod argument;
pub mod output;
pub mod request;
pub mod response;

use argument::ArgumentEncoding;
use async_trait::async_trait;
use output::{IntoRes, OutputEncoding};
use request::Req;
use response::Res;

use crate::argument::FromReq;

#[async_trait]
trait ServerFn<State, ResponseState, Request, Response>
where
    Response: Res<ResponseState> + Send + 'static,
    Request: Req<State> + Send + 'static,
    Self: FromReq<State, Request, Self::ArgumentEnc>,
{
    type Request;
    type ArgumentEnc: ArgumentEncoding<Request::Body>;
    type ResponseEnc: OutputEncoding<Request::Body>;
    type Output: IntoRes<Self::ResponseEnc, Response, ResponseState, Request::Body>;

    // the body of the fn
    fn call_fn_server(self) -> Self::Output;

    async fn respond_to_request(req: Request) -> Result<Response, ()> {
        let this = Self::from_req(req).await.map_err(|_| ())?; // TODO handle errors properly
        let output = this.call_fn_server();
        let res = output.into_res().map_err(|_| ())?; // TODO errors
        Ok(res)
    }
}
