use axum::{
    body::Body,
    http::{Request, Response},
    routing::get,
    Router,
};
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
        .route("/api/:name", get(server_fns::axum::handle_server_fn));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
