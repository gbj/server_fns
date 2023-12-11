use actix_web::{web, App, HttpServer};
use serde::{Deserialize, Serialize};
use server_fns::request::actix::ActixRequest;
use server_fns::response::actix::ActixResponse;
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
    type ServerRequest = ActixRequest;
    type ServerResponse = ActixResponse;
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            // prefixes all resources and routes attached to it...
            web::scope("/api").route("{name}", web::get().to(server_fns::actix::handle_server_fn)),
        )
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
