use alloy::hex;
use alloy::sol;
use alloy::sol_types::eip712_domain;
use alloy::sol_types::SolStruct;
use alloy::sol_types::SolValue;
use anyhow::Result;
use k256::ecdsa::{RecoveryId, Signature, VerifyingKey};
use oyster::{decode_attestation, get_attestation_doc, verify_with_timestamp};

use crate::client::{Request, Response};

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

pub struct OysterHttpsResponse {
    request: Request,
    response: Response,
    ip: String,
}

impl OysterHttpsResponse {
    pub fn new(request: Request, response: Response, ip: String) -> Self {
        Self { request, response, ip }
    }

    pub async fn verify(&self) -> Result<()> {
        // Verify attestation
        let attestation_doc = get_attestation_doc(format!("http://{}:1301/attestation/raw", self.ip).parse()?).await?;
        let parsed = decode_attestation(attestation_doc.clone())?;
        let expected_verifying_key = hex::encode(verify_with_timestamp(attestation_doc, parsed.pcrs, parsed.timestamp)?);
        // Create digest
        let digest = self._create_digest();
        // Verify signature
        let signature_with_recovery = hex::decode(&self.response.signature)?;
        let signature = Signature::from_slice(&signature_with_recovery[..64])?;
        let rec_id = RecoveryId::try_from(signature_with_recovery.last().unwrap().clone()-27)?;
        // Verify signature using secp256k1

        let recovered_key = VerifyingKey::recover_from_prehash(
            digest.as_slice(),
            &signature,
            rec_id
        )?;
        let recoverd_key_hex = hex::encode(recovered_key.to_encoded_point(false).as_bytes()).split_off(2);
        assert_eq!(expected_verifying_key, recoverd_key_hex);
        Ok(())
    }

    fn _sol_data(&self) -> RequestResponseData {
        RequestResponseData {
            requestData: RequestData {
                url: self.request.url.clone(),
                method: self.request.method.clone(),
                headerKeys: self.request.headers.keys().map(|k| k.to_owned()).collect(),
                headerValues: self.request.headers.values().map(|k| k.to_owned()).collect(),
                body: self.request.body.clone(),
                responseHeaders: self.request.response_headers.clone(),
            },
            responseData: ResponseData {
                handler: 1,
                status: self.response.status,
                headerKeys: self.response.headers.keys().map(|k| k.to_owned()).collect(),
                headerValues: self.response.headers.values().map(|k| k.to_owned()).collect(),
                body: self.response.body.clone(),
                timestamp: self.response.timestamp,
            },
        }
    }

    fn _create_digest(&self) -> Vec<u8> {
        // TODO: Implement digest creation
        let domain = eip712_domain! {
            name: "Teefetch",
            version: "1",
        };

        let signing_struct = self._sol_data();
        let signing_hash = signing_struct.eip712_signing_hash(&domain);

        signing_hash.to_vec()
    }

    pub fn abi_encode(&self) -> Result<Vec<u8>> {
        Ok(self._sol_data().abi_encode_params())
    }

    pub fn get_signature(&self) -> &str {
        &self.response.signature
    }
}
