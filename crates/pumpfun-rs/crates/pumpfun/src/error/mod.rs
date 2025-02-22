//! Error types for the Pump.fun SDK.
//!
//! This module defines the `ClientError` enum, which encompasses various error types that can occur when interacting with the Pump.fun program.
//! It includes specific error cases for bonding curve operations, metadata uploads, Solana client errors, and more.
//!
//! The `ClientError` enum provides a comprehensive set of error types to help developers handle and debug issues that may arise during interactions with the Pump.fun program.
//!
//! # Error Types
//!
//! - `BondingCurveNotFound`: The bonding curve account was not found.
//! - `BondingCurveError`: An error occurred while interacting with the bonding curve.
//! - `BorshError`: An error occurred while serializing or deserializing data using Borsh.
//! - `SolanaClientError`: An error occurred while interacting with the Solana RPC client.
//! - `UploadMetadataError`: An error occurred while uploading metadata to IPFS.
//! - `AnchorClientError`: An error occurred while interacting with the Anchor client.
//! - `InvalidInput`: Invalid input parameters were provided.
//! - `InsufficientFunds`: Insufficient funds for a transaction.
//! - `SimulationError`: Transaction simulation failed.
//! - `RateLimitExceeded`: Rate limit exceeded.

use anchor_client::solana_client;

#[derive(Debug)]
pub enum ClientError {
    /// Bonding curve account was not found
    BondingCurveNotFound,
    /// Error related to bonding curve operations
    BondingCurveError(&'static str),
    /// Error deserializing data using Borsh
    BorshError(std::io::Error),
    /// Error from Solana RPC client
    SolanaClientError(solana_client::client_error::ClientError),
    /// Error uploading metadata
    UploadMetadataError(Box<dyn std::error::Error>),
    /// Error from Anchor client
    AnchorClientError(anchor_client::ClientError),
    /// Invalid input parameters
    InvalidInput(&'static str),
    /// Insufficient funds for transaction
    InsufficientFunds,
    /// Transaction simulation failed
    SimulationError(String),
    /// Rate limit exceeded
    RateLimitExceeded,
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BondingCurveNotFound => write!(f, "Bonding curve not found"),
            Self::BondingCurveError(msg) => write!(f, "Bonding curve error: {}", msg),
            Self::BorshError(err) => write!(f, "Borsh serialization error: {}", err),
            Self::SolanaClientError(err) => write!(f, "Solana client error: {}", err),
            Self::UploadMetadataError(err) => write!(f, "Metadata upload error: {}", err),
            Self::AnchorClientError(err) => write!(f, "Anchor client error: {}", err),
            Self::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            Self::InsufficientFunds => write!(f, "Insufficient funds for transaction"),
            Self::SimulationError(msg) => write!(f, "Transaction simulation failed: {}", msg),
            Self::RateLimitExceeded => write!(f, "Rate limit exceeded"),
        }
    }
}

impl std::error::Error for ClientError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::BorshError(err) => Some(err),
            Self::SolanaClientError(err) => Some(err),
            Self::UploadMetadataError(err) => Some(err.as_ref()),
            Self::AnchorClientError(err) => Some(err),
            _ => None,
        }
    }
}
