pub mod argument;
pub mod output;
pub mod request;
pub mod response;

use argument::ArgumentEncoding;
use output::{IntoRes, OutputEncoding};
use request::Req;
use response::Res;

use crate::argument::FromReq;

trait ServerFn<State, Request, Response>
where
    Response: Res,
    Request: Req<State>,
    Self: FromReq<State, Request, Self::ArgumentEnc>,
{
    type Request;
    type ArgumentEnc: ArgumentEncoding;
    type ResponseEnc: OutputEncoding;
    type Output: IntoRes<Self::ResponseEnc, Response>;

    // the body of the fn
    fn call_fn_server(self) -> Self::Output;

    fn respond_to_request(req: Request) -> Result<Response, ()> {
        let this = Self::from_req(req).map_err(|_| ())?; // TODO handle errors properly
        let output = this.call_fn_server();
        let res = output.into_res().map_err(|_| ())?; // TODO errors
        Ok(res)
    }
}
