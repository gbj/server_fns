use serde::Serialize;

use crate::response::Res;

use super::{IntoRes, OutputEncoding};

/// Returns output as a JSON document encoded by [`serde_json`].
pub struct SerdeJson;

impl<E> OutputEncoding<E> for SerdeJson {
    const CONTENT_TYPE: &'static str = "application/json";

    type Error = serde_json::Error;
}

impl<T, Response, ErrorBody> IntoRes<SerdeJson, Response, ErrorBody> for T
where
    T: Serialize,
    Response: Res,
{
    fn into_res(self) -> Result<Response, <SerdeJson as OutputEncoding<T>>::Error> {
        let data = serde_json::to_string(&self)?;
        Ok(Response::from_string(data))
    }
}