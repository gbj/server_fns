use super::{Encoding, FromReq, FromRes};
use crate::error::ServerFnError;
use crate::request::{ClientReq, Req};
use crate::response::{ClientRes, Res};
use crate::{server_fn_error, IntoReq, IntoRes};
use serde::de::DeserializeOwned;
use serde::Serialize;
/// Pass arguments and receive responses as JSON in the body of a `POST` request.
pub struct SerdeJson;

impl super::Encoding for SerdeJson {
    const CONTENT_TYPE: &'static str = "application/json";
}

impl<T, Request> IntoReq<Request, SerdeJson> for T
where
    Request: ClientReq,
    T: Serialize + Send,
{
    fn into_req(self, path: &str) -> Result<Request, ServerFnError> {
        let data = serde_json::to_string(&self)?;
        Request::try_new_post(path, SerdeJson::CONTENT_TYPE, data)
    }
}

impl<T, Request> FromReq<Request, SerdeJson> for T
where
    Request: Req + Send + 'static,
    T: DeserializeOwned,
{
    async fn from_req(req: Request) -> Result<Self, ServerFnError> {
        let string_data = req.try_into_string().await?;
        serde_json::from_str::<Self>(&string_data).map_err(|e| ServerFnError::Args(e.to_string()))
    }
}

impl<T, Response> IntoRes<Response, SerdeJson> for T
where
    Response: Res,
    T: Serialize + Send,
{
    async fn into_res(self) -> Result<Response, ServerFnError> {
        let data = serde_json::to_string(&self)?;
        Response::try_from_string(SerdeJson::CONTENT_TYPE, data)
    }
}

impl<T, Response> FromRes<Response, SerdeJson> for T
where
    Response: ClientRes + Send,
    T: DeserializeOwned + Send,
{
    async fn from_res(res: Response) -> Result<Self, ServerFnError> {
        let data = res.try_into_string().await?;
        serde_json::from_str(&data).map_err(|e| ServerFnError::Deserialization(e.to_string()))
    }
}
impl From<serde_json::Error> for ServerFnError<serde_json::Error> {
    fn from(value: serde_json::Error) -> ServerFnError<serde_json::Error> {
        server_fn_error!(value)
    }
}
