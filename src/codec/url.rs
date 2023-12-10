use super::{Encoding, FromReq, IntoReq};
use crate::error::ServerFnError;
use crate::request::{ClientReq, Req};
use serde::de::DeserializeOwned;
use serde::Serialize;

/// Pass arguments as a URL-encoded query string of a `GET` request.
pub struct GetUrl;

/// Pass arguments as the URL-encoded body of a `POST` request.
pub struct PostUrl;

impl Encoding for GetUrl {
    const CONTENT_TYPE: &'static str = "application/x-www-form-urlencoded";
}

impl<T, Request> IntoReq<Request, GetUrl> for T
where
    Request: ClientReq,
    T: Serialize + Send,
{
    fn into_req(self, path: &str) -> Result<Request, ServerFnError> {
        let data = serde_qs::to_string(&self)?;
        Request::try_new_post(path, GetUrl::CONTENT_TYPE, data)
    }
}

impl<T, Request> FromReq<Request, GetUrl> for T
where
    Request: Req + Send + 'static,
    T: DeserializeOwned,
{
    async fn from_req(req: Request) -> Result<Self, ServerFnError> {
        let string_data = req.as_query().unwrap_or_default();
        let args = serde_qs::from_str::<Self>(string_data)
            .map_err(|e| ServerFnError::Args(e.to_string()))?;
        Ok(args)
    }
}

impl Encoding for PostUrl {
    const CONTENT_TYPE: &'static str = "application/x-www-form-urlencoded";
}

impl<T, Request> IntoReq<Request, PostUrl> for T
where
    Request: ClientReq,
    T: Serialize + Send,
{
    fn into_req(self, path: &str) -> Result<Request, ServerFnError> {
        let qs = serde_qs::to_string(&self)?;
        Request::try_new_post(path, PostUrl::CONTENT_TYPE, qs)
    }
}

impl<T, Request> FromReq<Request, PostUrl> for T
where
    Request: Req + Send + 'static,
    T: DeserializeOwned,
{
    async fn from_req(req: Request) -> Result<Self, ServerFnError> {
        let string_data = req.try_into_string().await?;
        let args = serde_qs::from_str::<Self>(&string_data)
            .map_err(|e| ServerFnError::Args(e.to_string()))?;
        Ok(args)
    }
}

/* #[async_trait]
impl<T, Request, Response> Codec<Request, Response, GetUrlJson> for T
where
    T: DeserializeOwned + Serialize + Send,
    Request: Req + Send,
    Response: Res + Send,
{
    async fn from_req(req: Request) -> Result<Self, ServerFnError> {
        let string_data = req.try_into_string()?;

        let args = serde_json::from_str::<Self>(&string_data)
            .map_err(|e| ServerFnError::Args(e.to_string()))?;
        Ok(args)
    }

    async fn into_req(self) -> Result<Request, ServerFnError> {
        /* let qs = serde_qs::to_string(&self)?;
        let req = http::Request::builder()
            .method("GET")
            .header(
                http::header::CONTENT_TYPE,
                <GetUrlJson as Encoding>::REQUEST_CONTENT_TYPE,
            )
            .body(Body::from(qs))?;
        Ok(req) */
        todo!()
    }

    async fn from_res(res: Response) -> Result<Self, ServerFnError> {
        todo!()
        /* let (_parts, body) = res.into_parts();

        let body_bytes = body
            .collect()
            .await
            .map(|c| c.to_bytes())
            .map_err(|e| ServerFnError::Deserialization(e.to_string()))?;
        let string_data = String::from_utf8(body_bytes.to_vec())?;
        serde_json::from_str(&string_data)
            .map_err(|e| ServerFnError::Deserialization(e.to_string())) */
    }

    async fn into_res(self) -> Response {
        // Need to catch and err or here, or handle Errors at a higher level
        let data = match serde_json::to_string(&self) {
            Ok(d) => d,
            Err(e) => return e.into_err_res(),
        };
        let builder = http::Response::builder();
        let res = builder
            .status(200)
            .header(
                http::header::CONTENT_TYPE,
                <GetUrlJson as Encoding>::RESPONSE_CONTENT_TYPE,
            )
            .body(Body::from(data))
            .unwrap();
        res
    }
}
 */
