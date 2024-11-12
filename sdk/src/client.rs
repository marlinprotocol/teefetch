use anyhow::Result;
use k256::ecdsa::{RecoveryId, Signature, VerifyingKey};
use oyster::{decode_attestation, get_attestation_doc, verify_with_timestamp};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct OysterHttpsClient {
    client: Client,
    ip: String,
}

impl OysterHttpsClient {
    pub fn new(ip: &str) -> Self {
        Self { client: reqwest::Client::new(), ip: ip.to_string() }
    }

    pub async fn oyster_fetch(&self, request: &Request) -> Result<Response> {
        let oyster_url = format!("https://{}:8080/", self.ip);
        let data = serde_json::to_string(&request).unwrap();
        let response = self.client.post(oyster_url).body(data).send().await?.text().await?;
        let response: Response = serde_json::from_str(&response)?;
        Ok(response)
    }
    pub async fn verify_signature(&self, request: &Request, response: &Response) -> Result<()> {
        // TODO: Implement signature verification
        // Verify attestation
        let attestation_doc = get_attestation_doc(format!("https://{}:1300/attestation", self.ip).parse()?).await?;
        let parsed = decode_attestation(attestation_doc.clone())?;
        let public_key = verify_with_timestamp(attestation_doc, parsed.pcrs, parsed.timestamp)?;
        // Create digest
        let digest = Self::create_digest(request);
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
        assert_eq!(public_key, recovered_key.to_sec1_bytes().to_vec());
        Ok(())
    }
    fn create_digest(request: &Request) -> Vec<u8> {
        // TODO: Implement digest creation
        "".as_bytes().to_vec()
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
    pub signature: String,
}