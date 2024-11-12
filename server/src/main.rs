use anyhow::Result;
use axum::routing::post;
use axum::Router;

async fn teefetch() {}

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new().route("/", post(teefetch));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
