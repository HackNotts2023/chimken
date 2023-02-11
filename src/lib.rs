use axum::response::Html;

pub mod db;

pub async fn index() -> Html<&'static str> {
    Html(include_str!("../templates/index.html"))
}