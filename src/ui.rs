use axum::body;
use axum::body::Empty;
use axum::body::Full;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Response;

use include_dir::include_dir;
use include_dir::Dir;

const STATIC_DIR: Dir<'static> = include_dir!("static/"); 

pub async fn index() -> Html<&'static str> {
    Html(include_str!("../templates/index.html"))
}

pub async fn statics(Path(path): Path<String>) -> impl IntoResponse  {
    let response = Response::builder();
    match STATIC_DIR.get_file(path.trim_start_matches('/')) {
        None => response.status(StatusCode::NOT_FOUND)
            .body(body::boxed(Empty::new())),
        Some(file) => response.status(StatusCode::OK)
            .body(body::boxed(Full::from(file.contents()))),
    }.unwrap_or_default()
}