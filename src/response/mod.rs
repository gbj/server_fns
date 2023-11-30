#[cfg(feature = "actix")]
pub mod http_response;
#[cfg(feature = "axum")]
pub mod response;

pub trait Res<ResponseBody> {}
