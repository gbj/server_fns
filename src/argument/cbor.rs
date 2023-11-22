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
impl<T, State, Request, StdErrorTrait, ErrorBody>
    FromReq<State, Request, StdErrorTrait, ErrorBody, PostCbor> for T
where
    T: DeserializeOwned,
    Request: Req<State, StdErrorTrait, ErrorBody> + Send + 'static,
    StdErrorTrait: std::error::Error,
    ciborium::de::Error<ErrorBody>: From<ciborium::de::Error<std::io::Error>>,
    ciborium::de::Error<ErrorBody>: From<StdErrorTrait>,
{
    async fn from_req(
        req: Request,
    ) -> Result<Self, <PostCbor as ArgumentEncoding<ErrorBody>>::Error> {
        let data = ciborium::de::from_reader(req.try_into_bytes().await?.as_ref())?;

        Ok(data)
    }
}
