//! Merkle tree implementation for the Tornado Cash Privacy Solution

use crate::{error::TornadoError, state::ROOT_HISTORY_SIZE};
use solana_program::{
    msg,
    program_error::ProgramError,
};
use sha3::{Digest, Keccak256};

/// Field size for BN254 curve
pub const FIELD_SIZE: [u8; 32] = [
    0x30, 0x64, 0x4e, 0x72, 0xe1, 0x31, 0xa0, 0x29, 0xb8, 0x5d, 0x12, 0x66, 0xb4, 0x1b, 0x4b, 0x30,
    0x73, 0xbe, 0x54, 0x46, 0xc3, 0x36, 0xb1, 0x0b, 0x51, 0x10, 0x5a, 0xf4, 0x00, 0x00, 0x00, 0x01,
];

/// Zero value for the Merkle tree (keccak256("tornado") % FIELD_SIZE)
pub const ZERO_VALUE: [u8; 32] = [
    0x2f, 0xe5, 0x4c, 0x60, 0xd3, 0xac, 0xab, 0xf3, 0x34, 0x3a, 0x35, 0xb6, 0xeb, 0xa1, 0x5d, 0xb4,
    0x82, 0x1b, 0x34, 0x0f, 0x76, 0xe7, 0x41, 0xe2, 0x24, 0x96, 0x85, 0xed, 0x48, 0x99, 0xaf, 0x6c,
];

/// Computes the hash of two leaves in the Merkle tree
pub fn hash_left_right(left: &[u8; 32], right: &[u8; 32]) -> Result<[u8; 32], ProgramError> {
    // Ensure inputs are within the field
    if !is_within_field(left) || !is_within_field(right) {
        return Err(TornadoError::InvalidMerkleTreeState.into());
    }

    // Compute MiMC(left, right) using Keccak256 as a substitute
    // In a real implementation, we would use MiMC or another zkSNARK-friendly hash function
    let mut hasher = Keccak256::new();
    hasher.update(left);
    hasher.update(right);
    let result = hasher.finalize();
    
    // Convert to array and ensure it's within the field
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result[..32]);
    
    // Ensure the result is within the field
    if !is_within_field(&hash) {
        // If not, take the result modulo the field size
        hash = mod_field_size(&hash);
    }
    
    Ok(hash)
}

/// Check if a value is within the BN254 field
fn is_within_field(value: &[u8; 32]) -> bool {
    for i in (0..32).rev() {
        if value[i] < FIELD_SIZE[i] {
            return true;
        }
        if value[i] > FIELD_SIZE[i] {
            return false;
        }
    }
    true
}

/// Take a value modulo the field size
fn mod_field_size(value: &[u8; 32]) -> [u8; 32] {
    // This is a simplified implementation
    // In a real implementation, we would use a proper big integer library
    let mut result = [0u8; 32];
    let mut carry = 0u16;
    
    for i in (0..32).rev() {
        let mut diff = value[i] as u16;
        if carry > 0 {
            diff += carry * 256;
            carry = 0;
        }
        if diff >= FIELD_SIZE[i] as u16 {
            diff -= FIELD_SIZE[i] as u16;
            carry = 1;
        }
        result[i] = diff as u8;
    }
    
    result
}

/// Get the zero value at a specific level in the Merkle tree
pub fn get_zero_value(level: usize) -> [u8; 32] {
    if level == 0 {
        return ZERO_VALUE;
    }
    
    // Pre-computed zero values for levels 1-31
    // These would be computed using hash_left_right(zeros(i-1), zeros(i-1))
    // For simplicity, we're using hardcoded values from the original contract
    match level {
        1 => [0x25, 0x6a, 0x61, 0x35, 0x77, 0x7e, 0xee, 0x2f, 0xd2, 0x6f, 0x54, 0xb8, 0xb7, 0x03, 0x7a, 0x25, 0x43, 0x9d, 0x52, 0x35, 0xca, 0xee, 0x22, 0x41, 0x54, 0x18, 0x6d, 0x2b, 0x8a, 0x52, 0xe3, 0x1d],
        2 => [0x11, 0x51, 0x94, 0x98, 0x95, 0xe8, 0x2a, 0xb1, 0x99, 0x24, 0xde, 0x92, 0xc4, 0x0a, 0x3d, 0x6f, 0x7b, 0xcb, 0x60, 0xd9, 0x2b, 0x00, 0x50, 0x4b, 0x81, 0x99, 0x61, 0x36, 0x83, 0xf0, 0xc2, 0x00],
        3 => [0x20, 0x12, 0x1e, 0xe8, 0x11, 0x48, 0x9f, 0xf8, 0xd6, 0x1f, 0x09, 0xfb, 0x89, 0xe3, 0x13, 0xf1, 0x49, 0x59, 0xa0, 0xf2, 0x8b, 0xb4, 0x28, 0xa2, 0x0d, 0xba, 0x6b, 0x0b, 0x06, 0x8b, 0x3b, 0xdb],
        // Add more levels as needed
        _ => {
            msg!("Warning: Zero value for level {} not pre-computed, using level 0", level);
            ZERO_VALUE
        }
    }
}

/// Insert a leaf into the Merkle tree
pub fn insert_leaf(
    leaf: &[u8; 32],
    current_index: u32,
    next_index: u32,
    height: u8,
    filled_subtrees: &mut [[u8; 32]],
    roots: &mut [[u8; 32]; ROOT_HISTORY_SIZE],
    current_root_index: &mut u8,
) -> Result<u32, ProgramError> {
    // Check if the tree is full
    if next_index >= 2u32.pow(height as u32) {
        return Err(TornadoError::MerkleTreeFull.into());
    }
    
    let mut current_idx = next_index;
    let mut current_level_hash = *leaf;
    
    // Update the tree
    for i in 0..height as usize {
        let left: [u8; 32];
        let right: [u8; 32];
        
        if current_idx % 2 == 0 {
            // If current_idx is even, the leaf is on the left
            left = current_level_hash;
            right = get_zero_value(i);
            filled_subtrees[i] = current_level_hash;
        } else {
            // If current_idx is odd, the leaf is on the right
            left = filled_subtrees[i];
            right = current_level_hash;
        }
        
        // Hash the left and right nodes
        current_level_hash = hash_left_right(&left, &right)?;
        current_idx /= 2;
    }
    
    // Update the root
    let new_root_index = (*current_root_index as usize + 1) % ROOT_HISTORY_SIZE;
    *current_root_index = new_root_index as u8;
    roots[new_root_index] = current_level_hash;
    
    Ok(next_index)
}

/// Check if a root is in the root history
pub fn is_known_root(
    root: &[u8; 32],
    roots: &[[u8; 32]; ROOT_HISTORY_SIZE],
    current_root_index: u8,
) -> bool {
    // Check if the root is zero
    if root.iter().all(|&x| x == 0) {
        return false;
    }
    
    let mut i = current_root_index as usize;
    loop {
        if root == &roots[i] {
            return true;
        }
        
        if i == 0 {
            i = ROOT_HISTORY_SIZE - 1;
        } else {
            i -= 1;
        }
        
        if i == current_root_index as usize {
            break;
        }
    }
    
    false
}

/// Get the last root
///
/// # Arguments
///
/// * `roots` - The roots history
/// * `current_root_index` - The current root index
///
/// # Returns
///
/// Returns the last root
pub fn get_last_root(
    roots: &[[u8; 32]; ROOT_HISTORY_SIZE],
    current_root_index: u8,
) -> [u8; 32] {
    roots[current_root_index as usize]
}

#[cfg(test)]
mod tests {
    // Unit tests will be added here
}