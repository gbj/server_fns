use rkyv::{
    de::deserializers::SharedDeserializeMap, ser::serializers::AllocSerializer,
    validation::validators::DefaultValidator, Archive, CheckBytes, Deserialize, Serialize,
};

use super::{FromReq, FromRes, IntoReq, IntoRes};
use crate::error::ServerFnError;
use crate::request::{ClientReq, Req};
use crate::response::{ClientRes, Res};
use bytes::Bytes;

/// Pass arguments and receive responses using `rkyv` in a `POST` request.
pub struct Rkyv;

const CONTENT_TYPE: &str = "application/rkyv";

impl<T, Request> IntoReq<Request, Rkyv> for T
where
    Request: Req + ClientReq,
    T: Serialize<AllocSerializer<1024>> + Send,
    T: Archive,
    T::Archived: for<'a> CheckBytes<DefaultValidator<'a>> + Deserialize<T, SharedDeserializeMap>,
{
    async fn into_req(self) -> Result<Request, ServerFnError> {
        let encoded = rkyv::to_bytes::<T, 1024>(&self)?;
        let bytes = Bytes::copy_from_slice(encoded.as_ref());
        Request::try_from_bytes("POST", CONTENT_TYPE, "", bytes).await
    }
}

impl<T, Request> FromReq<Request, Rkyv> for T
where
    Request: Req + Send + 'static,
    T: Serialize<AllocSerializer<1024>> + Send,
    T: Archive,
    T::Archived: for<'a> CheckBytes<DefaultValidator<'a>> + Deserialize<T, SharedDeserializeMap>,
{
    async fn from_req(req: Request) -> Result<Self, ServerFnError> {
        let body_bytes = req.try_into_bytes().await?;
        rkyv::from_bytes::<T>(&body_bytes).map_err(|e| ServerFnError::Args(e.to_string()))
    }
}

impl<T, Response> IntoRes<Response, Rkyv> for T
where
    Response: Res,
    T: Serialize<AllocSerializer<1024>> + Send,
    T: Archive,
    T::Archived: for<'a> CheckBytes<DefaultValidator<'a>> + Deserialize<T, SharedDeserializeMap>,
{
    async fn into_res(self) -> Result<Response, ServerFnError> {
        let encoded = rkyv::to_bytes::<T, 1024>(&self)
            .map_err(|e| ServerFnError::Serialization(e.to_string()))?;
        let bytes = Bytes::copy_from_slice(encoded.as_ref());
        Response::try_from_bytes(CONTENT_TYPE, bytes)
    }
}

impl<T, Response> FromRes<Response, Rkyv> for T
where
    Response: ClientRes + Send,
    T: Serialize<AllocSerializer<1024>> + Send,
    T: Archive,
    T::Archived: for<'a> CheckBytes<DefaultValidator<'a>> + Deserialize<T, SharedDeserializeMap>,
{
    async fn from_res(res: Response) -> Result<Self, ServerFnError> {
        let data = res.try_into_bytes()?;
        rkyv::from_bytes::<T>(&data).map_err(|e| ServerFnError::Deserialization(e.to_string()))
    }
}
