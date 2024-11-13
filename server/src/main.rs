use std::collections::HashMap;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use alloy::hex;
use alloy::sol;
use alloy::sol_types::eip712_domain;
use alloy::sol_types::SolStruct;
use anyhow::Result;
use axum::extract::State;
use axum::routing::post;
use axum::Json;
use axum::Router;
use clap::Parser;
use k256::ecdsa::SigningKey;
use reqwest::Client;
use reqwest::Method;
use reqwest::StatusCode;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize)]
struct Request {
    url: String,
    method: String,
    headers: HashMap<String, String>,
    excluded_headers: HashMap<String, String>,
    body: String,
    response_headers: Vec<String>,
}

#[derive(Deserialize, Serialize)]
struct Response {
    handler: u8,
    status: u16,
    headers: HashMap<String, String>,
    body: String,
    timestamp: u64,
    signature: String,
}

sol! {
    struct RequestData {
        string url;
        string method;
        string[] headerKeys;
        string[] headerValues;
        string body;
        string[] responseHeaders;
    }

    struct ResponseData {
        uint8 handler;
        uint16 status;
        string[] headerKeys;
        string[] headerValues;
        string body;
        uint64 timestamp;
    }

    struct RequestResponseData {
        RequestData requestData;
        ResponseData responseData;
    }
}

async fn teefetch(
    State(state): State<AppState>,
    Json(request): Json<Request>,
) -> Result<Json<Response>, StatusCode> {
    let client = Client::new();

    let mut req_builder = client
        .request(
            request
                .method
                .parse::<Method>()
                .map_err(|_| StatusCode::BAD_REQUEST)?,
            &request.url,
        )
        .body(request.body.clone());

    for (key, value) in request.headers.clone() {
        req_builder = req_builder.header(key, value);
    }

    for (key, value) in request.excluded_headers {
        req_builder = req_builder.header(key, value);
    }

    let response = req_builder
        .send()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let status = response.status().as_u16();
    let headers: HashMap<String, String> = response
        .headers()
        .iter()
        .filter(|(name, _)| request.response_headers.contains(&name.to_string()))
        .map(|(name, value)| {
            (
                name.to_string(),
                value.to_str().unwrap_or_default().to_string(),
            )
        })
        .collect();

    let body = response
        .text()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .as_secs() as u64;

    let domain = eip712_domain! {
        name: "Teefetch",
        version: "1",
    };
    let signing_struct = RequestResponseData {
        requestData: RequestData {
            url: request.url,
            method: request.method,
            headerKeys: request.headers.keys().map(|k| k.to_owned()).collect(),
            headerValues: request.headers.values().map(|k| k.to_owned()).collect(),
            body: request.body,
            responseHeaders: request.response_headers,
        },
        responseData: ResponseData {
            handler: 1,
            status,
            headerKeys: headers.keys().map(|k| k.to_owned()).collect(),
            headerValues: headers.values().map(|k| k.to_owned()).collect(),
            body: body.clone(),
            timestamp,
        },
    };
    let signing_hash = signing_struct.eip712_signing_hash(&domain);

    let signing_key = SigningKey::from_bytes(&state.secret.into())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (signature, recovery) = signing_key
        .sign_prehash_recoverable(signing_hash.as_slice())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(Response {
        handler: 1,
        status,
        headers,
        body,
        timestamp,
        signature: hex::encode_prefixed(signature.to_bytes())
            + &hex::encode(&[recovery.to_byte() + 27]),
    }))
}

#[derive(Clone)]
struct AppState {
    secret: [u8; 32],
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    secret: String,
    #[arg(short, long, default_value = "127.0.0.1:3000")]
    addr: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let secret: [u8; 32] = std::fs::read(args.secret)?.as_slice().try_into()?;

    let app = Router::new()
        .route("/json", post(teefetch))
        .with_state(AppState { secret });
    let listener = tokio::net::TcpListener::bind(args.addr).await?;

    axum::serve(listener, app).await?;

    Ok(())
}
