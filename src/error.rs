//! Error types for the Tornado Cash Privacy Solution

use num_derive::FromPrimitive;
use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

/// Errors that may be returned by the Tornado Cash program
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum TornadoError {
    /// Invalid instruction data
    #[error("Invalid instruction data")]
    InvalidInstructionData,

    /// Invalid account data
    #[error("Invalid account data")]
    InvalidAccountData,

    /// Account not initialized
    #[error("Account not initialized")]
    AccountNotInitialized,

    /// Account already initialized
    #[error("Account already initialized")]
    AccountAlreadyInitialized,

    /// Invalid Merkle tree state
    #[error("Invalid Merkle tree state")]
    InvalidMerkleTreeState,

    /// Merkle tree is full
    #[error("Merkle tree is full")]
    MerkleTreeFull,

    /// Invalid commitment
    #[error("Invalid commitment")]
    InvalidCommitment,

    /// Commitment already exists
    #[error("Commitment already exists")]
    CommitmentAlreadyExists,

    /// Invalid nullifier hash
    #[error("Invalid nullifier hash")]
    InvalidNullifierHash,

    /// Nullifier already spent
    #[error("Nullifier already spent")]
    NullifierAlreadySpent,

    /// Invalid Merkle root
    #[error("Invalid Merkle root")]
    InvalidMerkleRoot,

    /// Invalid proof
    #[error("Invalid proof")]
    InvalidProof,

    /// Invalid fee
    #[error("Invalid fee")]
    InvalidFee,

    /// Invalid recipient
    #[error("Invalid recipient")]
    InvalidRecipient,

    /// Invalid relayer
    #[error("Invalid relayer")]
    InvalidRelayer,

    /// Invalid amount
    #[error("Invalid amount")]
    InvalidAmount,

    /// Insufficient funds
    #[error("Insufficient funds")]
    InsufficientFunds,
}

impl From<TornadoError> for ProgramError {
    fn from(e: TornadoError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for TornadoError {
    fn type_of() -> &'static str {
        "TornadoError"
    }
}