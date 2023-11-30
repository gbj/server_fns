#[cfg(feature = "cbor")]
pub mod cbor;
#[cfg(feature = "manual")]
pub mod manual;
#[cfg(feature = "url_json")]
pub mod url_json;
#[cfg(feature = "json")]
pub mod json;

pub use crate::{error::ServerFnError, request::Req};
use async_trait::async_trait;
use crate::response::Res;
pub trait Encoding {
    const REQUEST_CONTENT_TYPE: &'static str;
    const RESPONSE_CONTENT_TYPE: &'static str;
}

#[async_trait]
pub trait Codec<RequestBody, ResponseBody, Request, Response, Enc>
    where
        Enc: Encoding,
        Request: Req<RequestBody> + Send,
        Response: Res<ResponseBody> + Send,
        RequestBody: Sync,
        ResponseBody: Sync,
        Self: Sized,{
    async fn from_req(req: Request) -> Result <Self, ServerFnError>;
    async fn into_req(self) -> Result<Request, ServerFnError>;

    async fn from_res(res: Response) -> Result<Self, ServerFnError>;
    async fn into_res(self) -> Result<Response, ServerFnError>;
}
