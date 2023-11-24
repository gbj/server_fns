use serde::Serialize;

use crate::{error::ServerFnError, response::Res};

use super::{IntoRes, OutputEncoding};

/// Returns output as a JSON document encoded by [`serde_json`].
pub struct SerdeJson;

impl OutputEncoding for SerdeJson {
    const CONTENT_TYPE: &'static str = "application/json";
}

impl<T, Response, State> IntoRes<SerdeJson, Response, State> for T
where
    T: Serialize,
    Response: Res<State>,
{
    fn into_res(self) -> Result<Response, ServerFnError> {
        let data = serde_json::to_string(&self)?;

        Ok(Response::try_into_string(data))
    }
}
