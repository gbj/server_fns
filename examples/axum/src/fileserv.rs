use axum::{
    body::Body,
    http::{Request, Response, StatusCode, Uri},
    response::{IntoResponse, Response as AxumResponse},
};
use tower::ServiceExt;
use tower_http::services::ServeDir;

pub async fn file_and_error_handler(uri: Uri, req: Request<Body>) -> AxumResponse {
    println!("looking for file at {uri:?}");
    get_static_file(uri.clone(), "target/site")
        .await
        .into_response()
}

async fn get_static_file(uri: Uri, root: &str) -> Result<Response<Body>, (StatusCode, String)> {
    let req = Request::builder()
        .uri(uri.clone())
        .body(Body::empty())
        .unwrap();
    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
    // This path is relative to the cargo root
    println!("root = {root}");
    match ServeDir::new(root).oneshot(req).await {
        Ok(res) => Ok(res.map(Body::new)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {err}"),
        )),
    }
}
