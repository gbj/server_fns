use super::{Encoding, FromReq};
use crate::error::ServerFnError;
use crate::request::browser::BrowserFormData;
use crate::request::{ClientReq, Req};
use crate::IntoReq;
use futures::StreamExt;
use multer::Multipart;
use web_sys::FormData;

pub struct MultipartFormData;

impl Encoding for MultipartFormData {
    const CONTENT_TYPE: &'static str = "multipart/form-data";
}

#[derive(Debug)]
pub enum MultipartData {
    Client(BrowserFormData),
    Server(multer::Multipart<'static>),
}

impl MultipartData {
    pub fn into_client_data(self) -> Option<BrowserFormData> {
        match self {
            MultipartData::Client(data) => Some(data),
            MultipartData::Server(_) => None,
        }
    }

    pub fn into_data(self) -> Option<Multipart<'static>> {
        match self {
            MultipartData::Client(_) => None,
            MultipartData::Server(data) => Some(data),
        }
    }
}

impl From<FormData> for MultipartData {
    fn from(value: FormData) -> Self {
        MultipartData::Client(value.into())
    }
}

impl<T, Request> IntoReq<Request, MultipartFormData> for T
where
    Request: ClientReq<FormData = BrowserFormData>,
    T: Into<MultipartData>,
{
    fn into_req(self, path: &str) -> Result<Request, ServerFnError> {
        let multi = self.into();
        Request::try_new_multipart(path, multi.into_client_data().unwrap())
    }
}

impl<T, Request> FromReq<Request, MultipartFormData> for T
where
    Request: Req + Send + 'static,
    T: From<MultipartData>,
{
    async fn from_req(req: Request) -> Result<Self, ServerFnError> {
        let boundary = req
            .to_content_type()
            .and_then(|ct| multer::parse_boundary(ct).ok())
            .expect("couldn't parse boundary");
        let stream = req.try_into_stream()?;
        let data =
            multer::Multipart::new(stream.map(|data| data.map_err(|e| e.to_string())), boundary);
        Ok(MultipartData::Server(data).into())
    }
}
