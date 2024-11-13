use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::response::OysterHttpsResponse;


pub struct OysterHttpsClient {
    client: Client,
    ip: String,
}

impl OysterHttpsClient {
    pub fn new(ip: &str) -> Self {
        Self { client: reqwest::Client::new(), ip: ip.to_string() }
    }

    pub async fn oyster_fetch(&self, request: Request) -> Result<OysterHttpsResponse> {
        let oyster_url = format!("http://{}:3000/json", self.ip);
        let data = serde_json::to_string(&request).unwrap();
        let response = self
            .client
            .post(oyster_url)
            .header("Content-Type", "application/json")
            .body(data)
            .send()
            .await?;

        // raise error if response is not ok
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Request failed with status: {}", response.status()));
        }

        let response_data: Response = response.json().await?;
        Ok(OysterHttpsResponse::new(request, response_data, self.ip.clone()))
    }
}

#[derive(Deserialize, Serialize)]
pub struct Request {
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub excluded_headers: HashMap<String, String>,
    pub body: String,
    pub excluded_body: String,
    pub response_headers: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Response {
    pub handler: u8,
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub timestamp: u64,
    pub signature: String,
}