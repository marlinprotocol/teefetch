# TEE Fetch (teeTLS)

A system for making verifiable HTTPS requests from within a Trusted Execution Environment (TEE) and proving the results on-chain.

## Overview

This project enables secure and verifiable HTTPS requests by executing them within a TEE, generating cryptographic proofs of the responses that can be verified by smart contracts on the blockchain. This creates a trusted bridge between web2 HTTPS endpoints and web3 smart contracts.

## Repository Structure

The repository consists of three main components:

- `/contracts` - Solidity smart contracts for on-chain verification of TEE responses
- `/enclave` - TEE environment configuration and setup
- `/sdk` - Rust SDK for interacting with the TEE service
- `/server` - The core TEE service implementation

## How It Works

1. A client makes an HTTPS request through the SDK
2. The request is forwarded to the TEE server
3. The TEE executes the HTTPS request in a secure environment
4. The response is signed with the TEE's attestation
5. The signed response can be verified on-chain using the smart contracts

## Components

### Smart Contracts (/contracts)

The smart contracts handle on-chain verification of TEE responses.

```solidity
// Example usage of Teefetch contract
contract MyContract {
    Teefetch public teefetch;

    function verifyHttpResponse(
        RequestResponseData calldata data,
        bytes calldata signature
    ) external {
        teefetch.verify(data, signature);
        // Response is now verified, process the data
    }
}
```

### SDK (/sdk)

The Rust SDK provides a client interface for making verifiable HTTPS requests.

```rust
use teefetch_sdk::OysterHttpsClient;

async fn example() {
    // Initialize client
    let client = OysterHttpsClient::new("localhost");
    
    // Make a verifiable request
    let request = Request {
        url: "https://api.example.com/data".to_string(),
        method: "GET".to_string(),
        headers: HashMap::new(),
        // ... other fields
    };
    
    let response = client.oyster_fetch(request).await?;
    
    // Verify the response
    response.verify().await?;
    
    // Get data for on-chain verification
    let encoded = response.abi_encode()?;
}
```

### TEE Server (/server)

The server runs inside a TEE and handles the actual HTTPS requests.

Configuration is managed through supervisord:
```ini
[program:teefetch]
command=/app/teefetch --secret /app/secp256k1.sec
autorestart=true
```

### Enclave Setup (/enclave)

The enclave component handles TEE configuration and attestation.

Key services:
- Attestation server (ports 1300/1301)
- TEE Fetch server (port 3000)
- DNS proxy for secure name resolution

## Getting Started

1. Setup the enclave environment:
```bash
cd enclave && ./setup.sh
```

2. Deploy the smart contracts:
```bash
cd contracts && forge script script/deploy/Teefetch.s.sol
```

3. Run the example SDK code:
```bash
cd sdk && cargo run --example example
```

## License

[Add license information]
