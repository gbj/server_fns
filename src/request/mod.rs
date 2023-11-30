#[cfg(feature = "actix")]
pub mod http_request;
#[cfg(feature = "axum")]
pub mod request;


pub trait Req<RequestBody> {}
