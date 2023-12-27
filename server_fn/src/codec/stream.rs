use std::pin::Pin;

use super::{Encoding, FromRes};
use crate::error::ServerFnError;
use crate::response::{ClientRes, Res};
use crate::IntoRes;
use bytes::Bytes;
use futures::{Stream, StreamExt};

pub struct Streaming;

impl Encoding for Streaming {
    const CONTENT_TYPE: &'static str = "application/octet-stream";
}

/* impl<T, Request> IntoReq<Request, ByteStream> for T
where
    Request: ClientReq,
    T: Stream<Item = Bytes> + Send,
{
    fn into_req(self, path: &str) -> Result<Request, ServerFnError> {
        Request::try_new_stream(path, ByteStream::CONTENT_TYPE, self)
    }
} */

/* impl<T, Request> FromReq<Request, ByteStream> for T
where
    Request: Req + Send + 'static,
    T: Stream<Item = Bytes> + Send,
{
    async fn from_req(req: Request) -> Result<Self, ServerFnError> {
        req.try_into_stream().await
    }
} */

pub struct ByteStream(Pin<Box<dyn Stream<Item = Result<Bytes, ServerFnError>> + Send>>);

impl ByteStream {
    pub fn into_inner(self) -> impl Stream<Item = Result<Bytes, ServerFnError>> + Send {
        self.0
    }
}

impl<S, T> From<S> for ByteStream
where
    S: Stream<Item = T> + Send + 'static,
    T: Into<Bytes>,
{
    fn from(value: S) -> Self {
        Self(Box::pin(value.map(|data| Ok(data.into()))))
    }
}

impl<Response> IntoRes<Response, Streaming> for ByteStream
where
    Response: Res,
{
    async fn into_res(self) -> Result<Response, ServerFnError> {
        Response::try_from_stream(Streaming::CONTENT_TYPE, self.into_inner())
    }
}

impl<Response> FromRes<Response, Streaming> for ByteStream
where
    Response: ClientRes + Send,
{
    async fn from_res(res: Response) -> Result<Self, ServerFnError> {
        let stream = res.try_into_stream()?;
        Ok(ByteStream(Box::pin(stream)))
    }
}

pub struct StreamingText;

impl Encoding for StreamingText {
    const CONTENT_TYPE: &'static str = "text/plain";
}

pub struct TextStream(Pin<Box<dyn Stream<Item = Result<String, ServerFnError>> + Send>>);

impl TextStream {
    pub fn into_inner(self) -> impl Stream<Item = Result<String, ServerFnError>> + Send {
        self.0
    }
}

impl<S, T> From<S> for TextStream
where
    S: Stream<Item = T> + Send + 'static,
    T: Into<String>,
{
    fn from(value: S) -> Self {
        Self(Box::pin(value.map(|data| Ok(data.into()))))
    }
}

impl<Response> IntoRes<Response, StreamingText> for TextStream
where
    Response: Res,
{
    async fn into_res(self) -> Result<Response, ServerFnError> {
        Response::try_from_stream(
            Streaming::CONTENT_TYPE,
            self.into_inner().map(|stream| stream.map(Into::into)),
        )
    }
}

impl<Response> FromRes<Response, StreamingText> for TextStream
where
    Response: ClientRes + Send,
{
    async fn from_res(res: Response) -> Result<Self, ServerFnError> {
        let stream = res.try_into_stream()?;
        Ok(TextStream(Box::pin(stream.map(|chunk| {
            chunk.and_then(|bytes| {
                String::from_utf8(bytes.into())
                    .map_err(|e| ServerFnError::Deserialization(e.to_string()))
            })
        }))))
    }
}
