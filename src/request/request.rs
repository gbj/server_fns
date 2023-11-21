use axum::body::HttpBody;
use futures::Stream;
use http::Request;
use crate::request::Req;

impl<B: HttpBody + Sized> Req<B> for Request<B>
{
    fn as_url(&self) -> &str {
        todo!()
    }

    fn into_string(self) -> String {
        todo!()
    }

    fn into_bytes(self) -> Vec<u8> {
        todo!()
    }

    fn into_stream(self) -> Box<dyn Stream<Item=B>> {
        todo!()
    }
}