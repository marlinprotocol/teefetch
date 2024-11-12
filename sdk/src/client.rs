use alloy::hex;
use alloy::sol;
use alloy::sol_types::eip712_domain;
use alloy::sol_types::SolStruct;
use anyhow::Result;
use k256::ecdsa::{RecoveryId, Signature, VerifyingKey};
use oyster::{decode_attestation, get_attestation_doc, verify_with_timestamp};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

pub struct OysterHttpsClient {
    client: Client,
    ip: String,
}

impl OysterHttpsClient {
    pub fn new(ip: &str) -> Self {
        Self { client: reqwest::Client::new(), ip: ip.to_string() }
    }

    pub async fn oyster_fetch(&self, request: &Request) -> Result<Response> {
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
        Ok(response_data)
    }
    pub async fn verify_signature(&self, request: &Request, response: &Response) -> Result<()> {
        // Verify attestation
        let attestation_doc = get_attestation_doc(format!("http://{}:1301/attestation/raw", self.ip).parse()?).await?;
        let parsed = decode_attestation(attestation_doc.clone())?;
        let expected_verifying_key = hex::encode(verify_with_timestamp(attestation_doc, parsed.pcrs, parsed.timestamp)?);
        // Create digest
        let digest = Self::create_digest(request, response);
        println!("{:?}", hex::encode(digest.clone()));
        // Verify signature
        let signature_with_recovery = hex::decode(&response.signature)?;
        let signature = Signature::from_slice(&signature_with_recovery[..64])?;
        let rec_id = RecoveryId::try_from(signature_with_recovery.last().unwrap().clone()-27)?;
        // Verify signature using secp256k1

        let recovered_key = VerifyingKey::recover_from_prehash(
            digest.as_slice(),
            &signature,
            rec_id
        )?;
        let recoverd_key_hex = hex::encode(recovered_key.to_encoded_point(false).as_bytes()).split_off(2);
        println!("recovered_key: {:?}", recoverd_key_hex);
        assert_eq!(expected_verifying_key, recoverd_key_hex);
        Ok(())
    }
    fn create_digest(request: &Request, response: &Response) -> Vec<u8> {
        // TODO: Implement digest creation
        let domain = eip712_domain! {
            name: "Teefetch",
            version: "1",
        };

        let signing_struct = RequestResponseData {
            requestData: RequestData {
                url: request.url.clone(),
                method: request.method.clone(),
                headerKeys: request.headers.keys().map(|k| k.to_owned()).collect(),
                headerValues: request.headers.values().map(|k| k.to_owned()).collect(),
                body: request.body.clone(),
                responseHeaders: request.response_headers.clone(),
            },
            responseData: ResponseData {
                handler: 1,
                status: response.status,
                headerKeys: response.headers.keys().map(|k| k.to_owned()).collect(),
                headerValues: response.headers.values().map(|k| k.to_owned()).collect(),
                body: response.body.clone(),
                timestamp: response.timestamp,
            },
        };
        let signing_hash = signing_struct.eip712_signing_hash(&domain);

        signing_hash.to_vec()
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