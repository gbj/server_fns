pub mod codec;
pub mod error;
pub mod request;
pub mod response;

use crate::codec::{Codec, Encoding};
use async_trait::async_trait;
use error::ServerFnError;
use request::Req;
use response::Res;

#[async_trait]
trait ServerFn<
    RequestBody,
    ResponseBody,
    Request,
    Response,
    IntoRequestBody,
    IntoResponseBody,
    IntoRequest,
    IntoResponse,
> where
    Request: Req<RequestBody> + Send + 'static,
    Response: Res<ResponseBody> + Send + 'static,
    RequestBody: Send + Sync,
    ResponseBody: Send + Sync,
    IntoRequestBody: Send + Sync,
    IntoResponseBody: Send + Sync,
    IntoRequest: Req<IntoRequestBody> + Send + Sync,
    IntoResponse: Res<IntoResponseBody> + Send + Sync,
    Self: Codec<
        RequestBody,
        ResponseBody,
        Request,
        Response,
        IntoRequestBody,
        IntoResponseBody,
        IntoRequest,
        IntoResponse,
        Self::Encoding,
    >,
{
    type Request;
    type Response;
    type Encoding: Encoding;
    type Output: Codec<
        RequestBody,
        ResponseBody,
        Request,
        Response,
        IntoRequestBody,
        IntoResponseBody,
        IntoRequest,
        IntoResponse,
        Self::Encoding,
    >;

    // the body of the fn
    fn call_fn_server(self) -> Self::Output;

    async fn respond_to_request(req: Request) -> Result<IntoResponse, ServerFnError> {
        let this = Self::from_req(req).await?;
        let output = this.call_fn_server();
        let res = output.into_res().await;
        Ok(res)
    }
}
