#[cfg(feature = "serde_json")]
pub mod serde_json;

use crate::response::Res;

pub trait OutputEncoding {
    const CONTENT_TYPE: &'static str;

    type Error;
}

pub trait IntoRes<Enc, Response>
where
    Enc: OutputEncoding,
    Response: Res,
    Self: Sized,
{
    fn into_res(self) -> Result<Response, Enc::Error>;
}
