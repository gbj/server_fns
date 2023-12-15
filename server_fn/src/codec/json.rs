use super::{Encoding, FromReq, FromRes};
use crate::error::ServerFnError;
use crate::request::{ClientReq, Req};
use crate::response::{ClientRes, Res};
use crate::{IntoReq, IntoRes};
use serde::de::DeserializeOwned;
use serde::Serialize;
/// Pass arguments and receive responses as JSON in the body of a `POST` request.
pub struct Json;

impl super::Encoding for Json {
    const CONTENT_TYPE: &'static str = "application/json";
}

impl<T, Request> IntoReq<Request, Json> for T
where
    Request: ClientReq,
    T: Serialize + Send,
{
    fn into_req(self, path: &str) -> Result<Request, ServerFnError> {
        let data = serde_json::to_string(&self)
            .map_err(|e| ServerFnError::Serialization(e.to_string()))?;
        Request::try_new_post(path, Json::CONTENT_TYPE, data)
    }
}

impl<T, Request> FromReq<Request, Json> for T
where
    Request: Req + Send + 'static,
    T: DeserializeOwned,
{
    async fn from_req(req: Request) -> Result<Self, ServerFnError> {
        let string_data = req.try_into_string().await?;
        serde_json::from_str::<Self>(&string_data).map_err(|e| ServerFnError::Args(e.to_string()))
    }
}

impl<T, Response> IntoRes<Response, Json> for T
where
    Response: Res,
    T: Serialize + Send,
{
    async fn into_res(self) -> Result<Response, ServerFnError> {
        let data = serde_json::to_string(&self)
            .map_err(|e| ServerFnError::Serialization(e.to_string()))?;
        Response::try_from_string(Json::CONTENT_TYPE, data)
    }
}

impl<T, Response> FromRes<Response, Json> for T
where
    Response: ClientRes + Send,
    T: DeserializeOwned + Send,
{
    async fn from_res(res: Response) -> Result<Self, ServerFnError> {
        let data = res.try_into_string().await?;
        serde_json::from_str(&data).map_err(|e| ServerFnError::Deserialization(e.to_string()))
    }
}
