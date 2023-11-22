#[cfg(feature = "ciborium")]
pub mod ciborium;
#[cfg(feature = "request")]
pub mod http_response;
#[cfg(feature = "request")]
pub mod response;
#[cfg(feature = "serde_json")]
pub mod serde_json;

use crate::response::Res;

pub trait OutputEncoding<ErrorBody> {
    const CONTENT_TYPE: &'static str;

    type Error;
}

pub trait IntoRes<Enc, Response, State, ErrorBody>
where
    Enc: OutputEncoding<ErrorBody>,
    Response: Res<State>,
    Self: Sized,
{
    fn into_res(self) -> Result<Response, Enc::Error>;
}
