//! State types for the Tornado Cash Privacy Solution

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

/// Maximum number of roots to store in history
pub const ROOT_HISTORY_SIZE: usize = 30;

/// Tornado instance state
#[derive(BorshSerialize, BorshDeserialize, Debug, Default, PartialEq)]
pub struct TornadoInstance {
    /// Is the instance initialized
    pub is_initialized: bool,
    /// The denomination amount for this instance
    pub denomination: u64,
    /// The height of the Merkle tree
    pub merkle_tree_height: u8,
    /// The Merkle tree account
    pub merkle_tree: Pubkey,
    /// The verifier account
    pub verifier: Pubkey,
}

impl Sealed for TornadoInstance {}

impl IsInitialized for TornadoInstance {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for TornadoInstance {
    const LEN: usize = 1 + 8 + 1 + 32 + 32; // is_initialized + denomination + merkle_tree_height + merkle_tree + verifier

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let instance = Self::try_from_slice(src)?;
        Ok(instance)
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let data = self.try_to_vec().unwrap();
        dst[..data.len()].copy_from_slice(&data);
    }
}

/// Merkle tree state
#[derive(BorshSerialize, BorshDeserialize, Debug, Default, PartialEq)]
pub struct MerkleTree {
    /// Is the tree initialized
    pub is_initialized: bool,
    /// The height of the tree
    pub height: u8,
    /// The current index in the tree
    pub current_index: u32,
    /// The next index to insert
    pub next_index: u32,
    /// The current root index
    pub current_root_index: u8,
    /// The roots history
    pub roots: [[u8; 32]; ROOT_HISTORY_SIZE],
    /// The filled subtrees
    pub filled_subtrees: Vec<[u8; 32]>,
    /// The nullifier hashes that have been used
    pub nullifier_hashes: Vec<[u8; 32]>,
    /// The commitments that have been used
    pub commitments: Vec<[u8; 32]>,
}

impl Sealed for MerkleTree {}

impl IsInitialized for MerkleTree {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl MerkleTree {
    /// Calculate the size of the Merkle tree account based on the height
    pub fn get_account_size(height: u8) -> usize {
        // Base size + filled_subtrees + nullifier_hashes + commitments
        // We allocate space for 2^height nullifiers and commitments
        let max_leaves = 2u32.pow(height as u32);
        1 + 1 + 4 + 4 + 1 + (ROOT_HISTORY_SIZE * 32) + (height as usize * 32) + (max_leaves as usize * 32) + (max_leaves as usize * 32)
    }
}