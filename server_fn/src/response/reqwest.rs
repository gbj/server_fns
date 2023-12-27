use super::ClientRes;
use crate::error::ServerFnError;
use bytes::Bytes;
use futures::{Stream, TryStreamExt};
use reqwest::Response;

impl ClientRes for Response {
    async fn try_into_string(self) -> Result<String, ServerFnError> {
        self.text()
            .await
            .map_err(|e| ServerFnError::Deserialization(e.to_string()))
    }

    async fn try_into_bytes(self) -> Result<Bytes, ServerFnError> {
        self.bytes()
            .await
            .map_err(|e| ServerFnError::Deserialization(e.to_string()))
    }

    fn try_into_stream(
        self,
    ) -> Result<impl Stream<Item = Result<Bytes, ServerFnError>> + Send + 'static, ServerFnError>
    {
        Ok(self
            .bytes_stream()
            .map_err(|e| ServerFnError::Response(e.to_string())))
    }
}
