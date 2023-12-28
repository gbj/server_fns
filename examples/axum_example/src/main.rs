use axum::{routing::get, Router};
use server_fn_macro_default::server;
use server_fns::error::ServerFnError;

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

// you can use the default setup
#[server(endpoint = "/my_server_fn", input = GetUrl)]
pub async fn my_server_fn(value: i32) -> Result<i32, ServerFnError> {
    println!("on server");
    Ok(value * 2)
}

// you can use any other error type
#[server]
pub async fn with_custom_error(value: i32) -> Result<i32, ServerFnError<std::io::Error>> {
    std::fs::read("./test.txt")?;
    Ok(value * 2)
}

// you can use a custom Result type alias
mod custom_res {
    use server_fn_macro_default::server;
    use server_fns::ServerFnError;
    type Result<T> = std::result::Result<T, ServerFnError>;

    #[server]
    pub async fn with_custom_result(value: i32) -> Result<i32> {
        Ok(value * 2)
    }
}
