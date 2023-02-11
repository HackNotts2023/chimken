use axum::Router;
use axum::routing;

use shuttle_service::ShuttleAxum;

use sync_wrapper::SyncWrapper;

pub mod db;
pub mod ui;

#[shuttle_service::main]
async fn axum() -> ShuttleAxum {
    Ok(SyncWrapper::new(Router::new()
        .route("/", routing::get(ui::index))
        .route("/static/*path", routing::get(ui::statics))))
}