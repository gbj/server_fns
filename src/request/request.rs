use axum::body::{Bytes};
use futures::Stream;
use http::Request;
use crate::request::Req;

impl<B: Sized> Req<B> for Request<B>
{
    fn as_url(&self) -> &str {
        todo!()
    }

    fn try_into_string(self) -> Result<String, Error> {
        self.into_body()
    }

    async fn try_into_bytes(self) -> Result<Bytes, Error> {
        hyper::body::to_bytes(self.into_body()).await
    }


    fn try_into_stream(self) -> dyn Stream<Item=B> {
        todo!()
    }
}