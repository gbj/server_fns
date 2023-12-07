use super::{FromReq, FromRes, IntoReq, IntoRes};
use crate::error::ServerFnError;
use crate::request::{ClientReq, Req};
use crate::response::{ClientRes, Res};
use serde::de::DeserializeOwned;
use serde::Serialize;

/// Pass arguments and receive responses as JSON in the body of a `POST` request.
pub struct SerdeJson;

const CONTENT_TYPE: &str = "application/json";

impl<T, Request> IntoReq<Request, SerdeJson> for T
where
    Request: Req + ClientReq,
    T: Serialize + Send,
{
    async fn into_req(self) -> Result<Request, ServerFnError> {
        let data = serde_json::to_string(&self)?;
        Request::try_from_string("POST", CONTENT_TYPE, "", data).await
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
        Response::try_from_string(CONTENT_TYPE, data)
    }
}

impl<T, Response> FromRes<Response, SerdeJson> for T
where
    Response: ClientRes + Send,
    T: DeserializeOwned + Send,
{
    async fn from_res(res: Response) -> Result<Self, ServerFnError> {
        let data = res.try_into_string()?;
        serde_json::from_str(&data).map_err(|e| ServerFnError::Deserialization(e.to_string()))
    }
}
