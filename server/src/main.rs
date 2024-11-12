use std::collections::HashMap;

use anyhow::Result;
use axum::routing::post;
use axum::Router;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize)]
struct Request {
    url: String,
    method: String,
    headers: HashMap<String, String>,
    excluded_headers: HashMap<String, String>,
    body: String,
    excluded_body: String,
    response_headers: Vec<String>,
}

#[derive(Deserialize, Serialize)]
struct Response {
    handler: u8,
    status: u16,
    headers: HashMap<String, String>,
    body: String,
    signature: String,
}

async fn teefetch() {}

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new().route("/json", post(teefetch));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}
