use anyhow::Result as AnyResult;

use axum::Router;
use axum::Server;

#[tokio::main]
async fn main() -> AnyResult<()> {
    let app = Router::new()
        .into_make_service();

    Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app)
        .await?;
    
    Ok(())
}