use serde::Serialize;

use crate::response::Res;

use super::{IntoRes, OutputEncoding};

/// Returns output as a CBOR binary encoded by [`ciborium`].
pub struct Ciborium;

impl OutputEncoding<T> for Ciborium {
    const CONTENT_TYPE: &'static str = "application/cbor";

    type Error = ciborium::ser::Error<T>;
}

impl<T, Response> IntoRes<Ciborium, Response> for T
    where
        T: Serialize,
        Response: Res,
{
    fn into_res(self) -> Result<Response, <Ciborium as OutputEncoding<T>>::Error> {
        let mut buffer: Vec<u8> = Vec::new();
        ciborium::ser::into_writer(&self, &mut buffer)?;

        Ok(Response::from_bytes(buffer))
    }
}
