use axum::response::Html;

pub async fn index() -> Html<&'static str> {
    Html("Hello world!")
}