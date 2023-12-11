use std::{future::Future, pin::Pin};

use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
    routing::get,
    Router,
};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use server_fns::{
    client::browser::BrowserClient, codec::json::SerdeJson, codec::url::GetUrl, ServerFn,
    ServerFnTraitObj,
};

// Start #[server] expansion
#[derive(Deserialize, Serialize)]
struct MyServerFn {
    foo: String,
    bar: f32,
}

impl ServerFn for MyServerFn {
    const PATH: &'static str = "/api/my_server_fn123";

    type Client = BrowserClient;
    type ServerRequest = Request<Body>;
    type ServerResponse = Response<Body>;
    type Output = f32;
    type InputEncoding = GetUrl;
    type OutputEncoding = SerdeJson;

    fn run_body(self) -> Self::Output {
        let MyServerFn { foo, bar } = self;
        foo.len() as f32 + bar
    }
}

inventory::submit! {
    ServerFnTraitObj::new(
        MyServerFn::PATH,
        |req| Box::pin(MyServerFn::run_on_server(req))
    )
}
// end #[server] expansion

/* Main fn */
#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/api/:name", get(handle_server_fn));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Start Axum integration
static REGISTERED_SERVER_FUNCTIONS: Lazy<
    DashMap<&'static str, ServerFnTraitObj<Request<Body>, Response<Body>>>,
> = {
    Lazy::new(|| {
        inventory::iter::<ServerFnTraitObj<Request<Body>, Response<Body>>>
            .into_iter()
            .map(|obj| (obj.path(), *obj))
            .collect()
    })
};

pub async fn handle_server_fn(req: Request<Body>) -> Response<Body> {
    let path = req.uri().path();
    // this is probably better done once by building a HashMap
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
