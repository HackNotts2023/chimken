use anyhow::Result as AnyResult;

use axum::Router;
use axum::Server;
use axum::routing;

use chimken::ui;

#[tokio::main]
async fn main() -> AnyResult<()> {
    let app = Router::new()
        .route("/", routing::get(ui::index))
        .into_make_service();
    Server::try_bind(&"0.0.0.0:8000".parse()?)?
        .serve(app)
        .await?;
    Ok(())
}