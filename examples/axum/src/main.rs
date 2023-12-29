#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{
        body::{Body, Bytes},
        response::Response,
        routing::{get, post},
        Router,
    };
    use axum_example::{app::*, fileserv::file_and_error_handler};
    use futures::StreamExt;
    use http::{header::CONTENT_TYPE, StatusCode};
    use tachys::{prelude::*, tachy_reaccy::Owner};
    use tokio::net::TcpListener;

    //simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    // build our application with a route
    let app = Router::new()
        .route("/", get(|| async move {
            let Root(owner, stream) = Root::global_ssr(move || {
                let app = view! {
                    <!DOCTYPE html>
                    <html>
                        <head>
                            <script type="module">
                                r#"import('/pkg/axum_example.js').then(m => m.default("/pkg/axum_example.wasm").then(() => m.hydrate()));"#
                            </script>
                        </head>
                        <body>
                            {axum_example::app::my_app()}
                        </body>
                    </html>
                };
                let app_stream = app
                    .to_html_stream_out_of_order();
                let shared_context = Owner::shared_context().expect("to have shared context");
                // TODO nonces
                let shared_context = shared_context.pending_data().unwrap().map(|chunk| format!("<script>{chunk}</script>"));
                // chained so we don't have <script> interspersed, interfering with hydration wak
                app_stream.chain(shared_context).inspect(|chunk| println!("{chunk}"))
                //futures::stream::select(app_stream, shared_context)
            });

            let stream = Body::from_stream(stream
                .chain(futures::stream::once(async move {
                    drop(owner);
                    Default::default()
                }))
                .map(|chunk| Ok::<Bytes, hyper::Error>(chunk.into())));

            Response::builder()
                .status(StatusCode::OK)
                .header(CONTENT_TYPE, "text/html")
                .body(stream)
                .expect("Failed to build response")
        }))
        .route("/api/*name", post(server_fns::axum::handle_server_fn))
        .route("/api/*name", get(server_fns::axum::handle_server_fn))
        //.leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = "127.0.0.1:3004";
    log::info!("listening on http://{}", &addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
