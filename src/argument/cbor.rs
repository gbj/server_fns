use super::{ArgumentEncoding, FromReq};
use crate::request::Req;
use async_trait::async_trait;
use serde::de::DeserializeOwned;

/// Pass argument as JSON in the body of a POST Request
pub struct PostCbor;

impl<ErrorBody> ArgumentEncoding<ErrorBody> for PostCbor {
    const CONTENT_TYPE: &'static str = "application/cbor";
    // Currently annoyed that these error types take generics
    type Error = ciborium::de::Error<ErrorBody>;
}
#[async_trait]
impl<T, State, Request> FromReq<State, Request, PostCbor> for T
where
    T: DeserializeOwned,
    Request: Req<State> + Send + 'static,
    ciborium::de::Error<Request::Body>: From<ciborium::de::Error<std::io::Error>>,
{
    async fn from_req(
        req: Request,
    ) -> Result<Self, <PostCbor as ArgumentEncoding<Request::Body>>::Error> {
        let data = ciborium::de::from_reader(req.try_into_bytes().await?.as_ref())?;

        Ok(data)
    }
}
