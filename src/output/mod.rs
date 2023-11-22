#[cfg(feature = "ciborium")]
pub mod ciborium;
#[cfg(feature = "request")]
pub mod http_response;
#[cfg(feature = "request")]
pub mod response;
#[cfg(feature = "serde_json")]
pub mod serde_json;

use crate::{error::ServerFnError, response::Res};

pub trait OutputEncoding {
    const CONTENT_TYPE: &'static str;
}

pub trait IntoRes<Enc, Response>
where
    Enc: OutputEncoding,
    Response: Res,
    Self: Sized,
{
    fn into_res(self) -> Result<Response, ServerFnError>;
}
