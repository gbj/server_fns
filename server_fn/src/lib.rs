pub mod client;
pub mod codec;
#[macro_use]
pub mod error;
pub mod request;
pub mod response;

use client::Client;
use codec::{Encoding, FromReq, FromRes, IntoReq, IntoRes};
use dashmap::DashMap;
pub use error::ServerFnError;
use once_cell::sync::Lazy;
use request::Req;
use response::Res;
use std::{future::Future, pin::Pin};

// reexports for the sake of the macro
#[doc(hidden)]
pub use const_format;
#[doc(hidden)]
pub use serde;
#[doc(hidden)]
pub use xxhash_rust;

pub trait ServerFn
where
    Self: Send
        + FromReq<Self::ServerRequest, Self::InputEncoding>
        + IntoReq<<Self::Client as Client>::Request, Self::InputEncoding>,
{
    const PATH: &'static str;

    /// The type of the HTTP client that will send the request from the client side.
    ///
    /// For example, this might be `gloo-net` in the browser, or `reqwest` for a desktop app.
    type Client: Client;

    /// The type of the HTTP request when received by the server function on the server side.
    type ServerRequest: Req + Send;

    /// The type of the HTTP response returned by the server function on the server side.
    type ServerResponse: Res + Send;

    /// The return type of the server function.
    ///
    /// This needs to be converted into `ServerResponse` on the server side, and converted
    /// *from* `ClientResponse` when received by the client.
    type Output: IntoRes<Self::ServerResponse, Self::OutputEncoding>
        + FromRes<<Self::Client as Client>::Response, Self::OutputEncoding>
        + Send;

    type InputEncoding: Encoding;
    type OutputEncoding: Encoding;

    // the body of the fn
    fn run_body(self) -> impl Future<Output = Result<Self::Output, ServerFnError>> + Send;

    fn run_on_server(
        req: Self::ServerRequest,
    ) -> impl Future<Output = Self::ServerResponse> + Send {
        async {
            Self::execute_on_server(req)
                .await
                .unwrap_or_else(Self::ServerResponse::error_response)
        }
    }

    fn run_on_client(self) -> impl Future<Output = Result<Self::Output, ServerFnError>> + Send {
        async move {
            let req = self.into_req(Self::PATH)?;
            let res = Self::Client::send(req).await?;
            let output = Self::Output::from_res(res).await?;
            Ok(output)
        }
    }

    #[doc(hidden)]
    fn execute_on_server(
        req: Self::ServerRequest,
    ) -> impl Future<Output = Result<Self::ServerResponse, ServerFnError>> + Send {
        async {
            let this = Self::from_req(req).await?;
            let output = this.run_body().await?;
            let res = output.into_res().await?;
            Ok(res)
        }
    }

    fn url() -> &'static str {
        Self::PATH
    }
}

#[doc(hidden)]
pub use inventory;

#[macro_export]
macro_rules! initialize_server_fn_map {
    ($req:ty, $res:ty) => {
        once_cell::sync::Lazy::new(|| {
            $crate::inventory::iter::<ServerFnTraitObj<$req, $res>>
                .into_iter()
                .map(|obj| (obj.path(), *obj))
                .collect()
        })
    };
}

pub struct ServerFnTraitObj<Req, Res> {
    path: &'static str,
    handler: fn(Req) -> Pin<Box<dyn Future<Output = Res> + Send>>,
}

impl<Req, Res> ServerFnTraitObj<Req, Res> {
    pub const fn new(
        path: &'static str,
        handler: fn(Req) -> Pin<Box<dyn Future<Output = Res> + Send>>,
    ) -> Self {
        Self { path, handler }
    }

    pub fn path(&self) -> &'static str {
        self.path
    }

    pub async fn run(&self, req: Req) -> Res {
        (self.handler)(req).await
    }
}

impl<Req, Res> Clone for ServerFnTraitObj<Req, Res> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Req, Res> Copy for ServerFnTraitObj<Req, Res> {}

type LazyServerFnMap<Req, Res> = Lazy<DashMap<&'static str, ServerFnTraitObj<Req, Res>>>;

// Axum integration
#[cfg(feature = "axum")]
pub mod axum {
    use crate::{LazyServerFnMap, ServerFn, ServerFnTraitObj};
    use axum::body::Body;
    use http::{Request, Response, StatusCode};

    inventory::collect!(ServerFnTraitObj<Request<Body>, Response<Body>>);

    static REGISTERED_SERVER_FUNCTIONS: LazyServerFnMap<Request<Body>, Response<Body>> =
        initialize_server_fn_map!(Request<Body>, Response<Body>);

    pub fn register_explicit<T>()
    where
        T: ServerFn<ServerRequest = Request<Body>, ServerResponse = Response<Body>> + 'static,
    {
        REGISTERED_SERVER_FUNCTIONS.insert(
            T::PATH,
            ServerFnTraitObj::new(T::PATH, |req| Box::pin(T::run_on_server(req))),
        );
    }

    pub async fn handle_server_fn(req: Request<Body>) -> Response<Body> {
        let path = req.uri().path();

        if let Some(server_fn) = REGISTERED_SERVER_FUNCTIONS.get(path) {
            server_fn.run(req).await
        } else {
            Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(format!(
                "Could not find a server function at the route {path}. \n\nIt's likely that either\n 1. The API prefix you specify in the `#[server]` macro doesn't match the prefix at which your server function handler is mounted, or \n2. You are on a platform that doesn't support automatic server function registration and you need to call ServerFn::register_explicit() on the server function type, somewhere in your `main` function.",
            )))
            .unwrap()
        }
    }
}

// Actix integration
#[cfg(feature = "actix")]
pub mod actix {
    use actix_web::{HttpRequest, HttpResponse};
    use send_wrapper::SendWrapper;

    use crate::request::actix::ActixRequest;
    use crate::response::actix::ActixResponse;
    use crate::{LazyServerFnMap, ServerFn, ServerFnTraitObj};

    inventory::collect!(ServerFnTraitObj<ActixRequest, ActixResponse>);

    static REGISTERED_SERVER_FUNCTIONS: LazyServerFnMap<ActixRequest, ActixResponse> =
        initialize_server_fn_map!(ActixRequest, ActixResponse);

    pub fn register_explicit<T>()
    where
        T: ServerFn<ServerRequest = ActixRequest, ServerResponse = ActixResponse> + 'static,
    {
        REGISTERED_SERVER_FUNCTIONS.insert(
            T::PATH,
            ServerFnTraitObj::new(T::PATH, |req| Box::pin(T::run_on_server(req))),
        );
    }

    pub async fn handle_server_fn(req: HttpRequest) -> HttpResponse {
        let path = req.uri().path();
        if let Some(server_fn) = REGISTERED_SERVER_FUNCTIONS.get(path) {
            server_fn
                .run(ActixRequest(SendWrapper::new(req)))
                .await
                .0
                .take()
        } else {
            HttpResponse::BadRequest().body(format!(
                "Could not find a server function at the route {path}. \n\nIt's likely that either\n 1. The API prefix you specify in the `#[server]` macro doesn't match the prefix at which your server function handler is mounted, or \n2. You are on a platform that doesn't support automatic server function registration and you need to call ServerFn::register_explicit() on the server function type, somewhere in your `main` function.",
            ))
        }
    }
}
