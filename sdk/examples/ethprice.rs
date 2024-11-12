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
    let request = Request {
        url: "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd".to_string(),
        method: "GET".to_string(),
        headers: HashMap::new(),
        excluded_headers: HashMap::new(),
        body: "".to_string(),
        excluded_body: "".to_string(),
        response_headers: vec!["Content-Type".to_string()],
    };
    let response = client.oyster_fetch(&request).await.unwrap();
    println!("{:?}", serde_json::to_string(&response).unwrap());
    client.verify_signature(&request, &response).await.unwrap();
}