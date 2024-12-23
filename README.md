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

## Getting Started

See individual README files in each component directory for specific setup and usage instructions:

- Smart Contracts: `/contracts/README.md`
- SDK Usage: `/sdk/examples/example.rs`
- Server Setup: `/enclave/setup.sh`

## License

[Add license information]
