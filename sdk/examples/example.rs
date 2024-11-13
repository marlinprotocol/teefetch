use alloy::hex;
use clap::Parser;
use oyster_https::OysterHttpsClient;
use oyster_https::Request;
use std::collections::HashMap;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long, value_parser)]
    ip: String,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    // TODO: Change to your Oyster device IP address
    let client = OysterHttpsClient::new(&args.ip);
    let mut headers = HashMap::new();
    headers.insert("Host".to_string(), "example.com".to_string());
    let request = Request {
        url: "https://example.com".to_string(),
        method: "GET".to_string(),
        headers,
        excluded_headers: HashMap::new(),
        body: "".to_string(),
        excluded_body: "".to_string(),
        response_headers: vec![],
    };
    let response = client.oyster_fetch(request).await.unwrap();
    response.verify().await.unwrap();
    println!("Response verified");
    println!(
        "RequestResponseData: \n{:?}",
        hex::encode(response.abi_encode().unwrap())
    );
    println!("Signature: \n{:?}", response.get_signature());
}
