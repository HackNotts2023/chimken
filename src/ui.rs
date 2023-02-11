use axum::body;
use axum::body::Empty;
use axum::body::Full;
use axum::http::StatusCode;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::response::Response;

use include_dir::include_dir;
use include_dir::Dir;

use minijinja::render;
use minijinja::Environment;

pub const STATIC_DIR: Dir<'static> = include_dir!("static/");

pub const TEMPLATE_DIR: Dir<'static> = include_dir!("templates/");

pub async fn index() -> Html<String> {
    Html(render!(in template_env(), include_str!("../templates/index.html")))
}

pub async fn template_path(path: String) -> impl IntoResponse {
    let response = Response::builder();
    match TEMPLATE_DIR.get_file(path.trim_start_matches('/')) {
        None => response.status(StatusCode::NOT_FOUND)
            .body(body::boxed(Empty::new())),
        Some(file) => response.status(StatusCode::OK)
            .body(body::boxed(Full::from(render!(in template_env(), file.contents_utf8().unwrap_or_default())))),
    }.unwrap_or_default()
}

pub async fn static_path(path: String) -> impl IntoResponse  {
    let response = Response::builder();
    match STATIC_DIR.get_file(path.trim_start_matches('/')) {
        None => response.status(StatusCode::NOT_FOUND)
            .body(body::boxed(Empty::new())),
        Some(file) => response.status(StatusCode::OK)
            .body(body::boxed(Full::from(render!(in template_env(), file.contents_utf8().unwrap_or_default())))),
    }.unwrap_or_default()
}

fn template_env() -> Environment<'static> {
    TEMPLATE_DIR.files()
        .filter(|file| file.path().extension().and_then(|ext| ext.to_str()) == Some("html"))
        .try_fold(Environment::new(), |mut env, file| env.add_template(file.path()
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or_default(), file.contents_utf8().unwrap_or_default())
            .ok()
            .map(|_| env))
        .unwrap_or_default()
}