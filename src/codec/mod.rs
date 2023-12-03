#[cfg(feature = "cbor")]
pub mod cbor;
#[cfg(feature = "json")]
pub mod json;
#[cfg(feature = "rkyv")]
pub mod rkyv;
#[cfg(feature = "url_json")]
pub mod url_json;
use crate::response::Res;
pub use crate::{error::ServerFnError, request::Req};
use async_trait::async_trait;
pub trait Encoding {
    const REQUEST_CONTENT_TYPE: &'static str;
    const RESPONSE_CONTENT_TYPE: &'static str;
}

#[async_trait]
pub trait Codec<
    RequestBody,
    ResponseBody,
    Request,
    Response,
    IntoRequestBody,
    IntoResponseBody,
    IntoRequest,
    IntoResponse,
    Enc,
> where
    Enc: Encoding,
    Request: Req<RequestBody> + Send,
    Response: Res<ResponseBody> + Send,
    IntoRequest: Req<IntoRequestBody> + Send,
    IntoResponse: Res<IntoResponseBody> + Send,
    Self: Sized,
{
    async fn from_req(req: Request) -> Result<Self, ServerFnError>;
    async fn into_req(self) -> Result<IntoRequest, ServerFnError>;

    async fn from_res(res: Response) -> Result<Self, ServerFnError>;
    async fn into_res(self) -> IntoResponse;
}
