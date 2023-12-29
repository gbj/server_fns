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

    println!("here we are");

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// you can use the default setup
#[server(endpoint = "/a", input = GetUrl)]
pub async fn a(value: i32) -> Result<i32, ServerFnError> {
    println!("on server");
    Ok(value * 2)
}

// you can use any other error type
#[server(endpoint = "/b", input = GetUrl)]
pub async fn b() -> Result<(), ServerFnError<std::io::Error>> {
    std::fs::read("./test.txt")?;
    Ok(())
}

#[server(endpoint = "/c", input = GetUrl)]
pub async fn c(value: i32) -> Result<i32, ServerFnError<std::io::Error>> {
    Ok(value * 2)
}

// you can use a custom Result type alias
mod custom_res {
    use server_fn_macro_default::server;
    use server_fns::ServerFnError;
    type Result<T> = std::result::Result<T, ServerFnError>;

    #[server(endpoint = "/d", input = GetUrl)]
    pub async fn d(value: i32) -> Result<i32> {
        Ok(value * 2)
    }
}
