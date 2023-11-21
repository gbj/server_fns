use axum::body::{Bytes};
use futures::Stream;
use http::Request;
use crate::request::Req;

impl<B: Sized, E> Req<B, E> for Request<B>
{
    fn as_url(&self) -> &str {
        todo!()
    }

    async fn try_into_string(self) -> Result<String, Error> {
        let body_bytes = hyper::body::to_bytes(self.into_body()).await?;
       Ok(String::from_utf8(body_bytes.to_vec())?)
    }

    async fn try_into_bytes(self) -> Result<Bytes, Error> {
        hyper::body::to_bytes(self.into_body()).await
    }


    fn try_into_stream(self) -> dyn Stream<Item=B> {
        todo!()
    }
}