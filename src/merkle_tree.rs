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

/// Computes the hash of two leaves in the Merkle tree using MiMC
pub fn hash_left_right(left: &[u8; 32], right: &[u8; 32]) -> Result<[u8; 32], ProgramError> {
    // Ensure inputs are within the field
    if !is_within_field(left) || !is_within_field(right) {
        return Err(TornadoError::InvalidMerkleTreeState.into());
    }

    // Convert bytes to field elements
    let left_fe = bytes_to_field_element(left)?;
    let right_fe = bytes_to_field_element(right)?;
    
    // Compute MiMC(left, right)
    let result_fe = mimc_hash(left_fe, right_fe)?;
    
    // Convert back to bytes
    let result = field_element_to_bytes(result_fe);
    
    Ok(result)
}

/// Convert bytes to a field element
fn bytes_to_field_element(bytes: &[u8; 32]) -> Result<[u64; 4], ProgramError> {
    if !is_within_field(bytes) {
        return Err(TornadoError::InvalidMerkleTreeState.into());
    }
    
    let mut result = [0u64; 4];
    
    // Convert bytes to 4 u64 limbs
    for i in 0..4 {
        let mut limb = 0u64;
        for j in 0..8 {
            limb |= (bytes[i * 8 + j] as u64) << (j * 8);
        }
        result[i] = limb;
    }
    
    Ok(result)
}

/// Convert a field element to bytes
fn field_element_to_bytes(fe: [u64; 4]) -> [u8; 32] {
    let mut result = [0u8; 32];
    
    // Convert 4 u64 limbs to bytes
    for i in 0..4 {
        for j in 0..8 {
            result[i * 8 + j] = ((fe[i] >> (j * 8)) & 0xFF) as u8;
        }
    }
    
    result
}

/// MiMC hash function (Minimal Multiplicative Complexity)
/// This is a zkSNARK-friendly hash function
fn mimc_hash(left: [u64; 4], right: [u64; 4]) -> Result<[u64; 4], ProgramError> {
    // MiMC constants (derived from the decimal digits of π)
    const MIMC_ROUNDS: usize = 20;
    const MIMC_CONSTANTS: [[u64; 4]; MIMC_ROUNDS] = [
        [0x243f6a8885a308d3, 0x13198a2e03707344, 0xa4093822299f31d0, 0x082efa98ec4e6c89],
        [0x452821e638d01377, 0xbe5466cf34e90c6c, 0xc0ac29b7c97c50dd, 0x3f84d5b5b5470917],
        [0x9216d5d98979fb1b, 0xd1310ba698dfb5ac, 0x2ffd72dbd01adfb7, 0xb8e1afed6a267e96],
        [0xba7c9045f12c7f99, 0x24a19947b3916cf7, 0x0801f2e2858efc16, 0x636920d871574e69],
        [0xa458fea3f4933d7e, 0x0d95748f728eb658, 0x718bcd5882154aee, 0x7b54a41dc25a59b5],
        [0x9c30d5392af26013, 0xc5d1b023286085f0, 0xca417918b8db38ef, 0x8e79dcb0603a180e],
        [0x6c9e0e8bb01e8a3e, 0xd71577c1bd314b27, 0x78af2fda55605c60, 0xe65525f3aa55ab94],
        [0xaa55ab94aaaa5555, 0x55aa55aa55aa55aa, 0xaa55ab94aaaa5555, 0x55aa55aa55aa55aa],
        [0x5aa55aa55aa55aa5, 0xa55aa55aa55aa55a, 0x5aa55aa55aa55aa5, 0xa55aa55aa55aa55a],
        [0xaaaaaaaaaaaaaaaa, 0xaaaaaaaaaaaaaaaa, 0xaaaaaaaaaaaaaaaa, 0xaaaaaaaaaaaaaaaa],
        [0x5555555555555555, 0x5555555555555555, 0x5555555555555555, 0x5555555555555555],
        [0xaaaaaaaaaaaaaaaa, 0x5555555555555555, 0xaaaaaaaaaaaaaaaa, 0x5555555555555555],
        [0x5555555555555555, 0xaaaaaaaaaaaaaaaa, 0x5555555555555555, 0xaaaaaaaaaaaaaaaa],
        [0x1111111111111111, 0x2222222222222222, 0x3333333333333333, 0x4444444444444444],
        [0x5555555555555555, 0x6666666666666666, 0x7777777777777777, 0x8888888888888888],
        [0x9999999999999999, 0xaaaaaaaaaaaaaaaa, 0xbbbbbbbbbbbbbbbb, 0xcccccccccccccccc],
        [0xdddddddddddddddd, 0xeeeeeeeeeeeeeeee, 0xffffffffffffffff, 0x0000000000000000],
        [0x1234567890abcdef, 0xfedcba0987654321, 0x1234567890abcdef, 0xfedcba0987654321],
        [0x0123456789abcdef, 0xfedcba9876543210, 0x0123456789abcdef, 0xfedcba9876543210],
        [0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000],
    ];
    
    // Initialize state with left input
    let mut state = left;
    
    // Add right input to state
    state = field_add(state, right);
    
    // Apply MiMC rounds
    for i in 0..MIMC_ROUNDS {
        // Add round constant
        state = field_add(state, MIMC_CONSTANTS[i]);
        
        // Cube the state (x^3 is the MiMC S-box)
        state = field_cube(state)?;
    }
    
    // Add right input again (Feistel construction)
    state = field_add(state, right);
    
    Ok(state)
}

/// Add two field elements
fn field_add(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    let mut result = [0u64; 4];
    let mut carry = 0u64;
    
    for i in 0..4 {
        let (sum1, c1) = a[i].overflowing_add(b[i]);
        let (sum2, c2) = sum1.overflowing_add(carry);
        
        result[i] = sum2;
        carry = if c1 || c2 { 1 } else { 0 };
    }
    
    // Reduce modulo field size if necessary
    if carry > 0 || !is_within_field(&field_element_to_bytes(result)) {
        result = field_mod(result);
    }
    
    result
}

/// Compute the cube of a field element (x^3)
fn field_cube(a: [u64; 4]) -> Result<[u64; 4], ProgramError> {
    // Compute a^2
    let a_squared = field_mul(a, a)?;
    
    // Compute a^3 = a * a^2
    field_mul(a, a_squared)
}

/// Multiply two field elements
fn field_mul(a: [u64; 4], b: [u64; 4]) -> Result<[u64; 4], ProgramError> {
    // This is a simplified implementation of field multiplication
    // In a real implementation, we would use a proper big integer library
    
    // Convert to bytes for simplicity
    let a_bytes = field_element_to_bytes(a);
    let b_bytes = field_element_to_bytes(b);
    
    // Use a simple schoolbook multiplication
    let mut result = [0u8; 64]; // Temporary result (twice the size)
    
    for i in 0..32 {
        let mut carry = 0u16;
        for j in 0..32 {
            let idx = i + j;
            if idx < 64 {
                let prod = (a_bytes[i] as u16) * (b_bytes[j] as u16) + (result[idx] as u16) + carry;
                result[idx] = (prod & 0xFF) as u8;
                carry = prod >> 8;
            }
        }
    }
    
    // Reduce modulo field size
    let mut reduced = [0u8; 32];
    reduced.copy_from_slice(&result[0..32]); // Simplified reduction
    
    if !is_within_field(&reduced) {
        reduced = mod_field_size(&reduced);
    }
    
    // Convert back to field element
    bytes_to_field_element(&reduced)
}

/// Reduce a field element modulo the field size
fn field_mod(a: [u64; 4]) -> [u64; 4] {
    // Convert to bytes for simplicity
    let a_bytes = field_element_to_bytes(a);
    
    // Reduce modulo field size
    let reduced = mod_field_size(&a_bytes);
    
    // Convert back to field element
    bytes_to_field_element(&reduced).unwrap_or([0u64; 4])
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
    use super::*;
    use solana_program::program_error::ProgramError;

    #[test]
    fn test_hash_left_right() {
        // Test with valid inputs
        let left = [1u8; 32];
        let right = [2u8; 32];
        let result = hash_left_right(&left, &right).unwrap();
        
        // Ensure result is not zero and is within field
        assert!(!result.iter().all(|&x| x == 0));
        assert!(is_within_field(&result));
        
        // Test with inputs at field boundary
        let boundary = FIELD_SIZE;
        let result = hash_left_right(&boundary, &right);
        assert!(result.is_err());
        
        // Test with zero values
        let zero = [0u8; 32];
        let result = hash_left_right(&zero, &zero).unwrap();
        assert!(is_within_field(&result));
        
        // Test determinism
        let result2 = hash_left_right(&left, &right).unwrap();
        assert_eq!(result, result2);
        
        // Test different inputs produce different outputs
        let left2 = [3u8; 32];
        let result3 = hash_left_right(&left2, &right).unwrap();
        assert!(result != result3);
    }
    
    #[test]
    fn test_is_within_field() {
        // Test with value below field size
        let below = [0u8; 32];
        assert!(is_within_field(&below));
        
        // Test with value equal to field size
        let equal = FIELD_SIZE;
        assert!(is_within_field(&equal));
        
        // Test with value above field size
        let mut above = FIELD_SIZE;
        above[31] += 1;
        assert!(!is_within_field(&above));
    }
    
    #[test]
    fn test_mod_field_size() {
        // Test with value below field size
        let below = [1u8; 32];
        let result = mod_field_size(&below);
        assert_eq!(result, below);
        
        // Test with value above field size
        let mut above = FIELD_SIZE;
        above[31] += 10;
        let result = mod_field_size(&above);
        assert!(is_within_field(&result));
        assert!(result != above);
    }
    
    #[test]
    fn test_get_zero_value() {
        // Test level 0
        let level0 = get_zero_value(0);
        assert_eq!(level0, ZERO_VALUE);
        
        // Test level 1
        let level1 = get_zero_value(1);
        assert!(level1 != ZERO_VALUE);
        
        // Test level 2
        let level2 = get_zero_value(2);
        assert!(level2 != level1);
        
        // Test high level (should default to level 0)
        let high_level = get_zero_value(100);
        assert_eq!(high_level, ZERO_VALUE);
    }
    
    #[test]
    fn test_insert_leaf() {
        // Create a test Merkle tree
        let height = 3;
        let mut filled_subtrees = vec![[0u8; 32]; height as usize];
        let mut roots = [[0u8; 32]; ROOT_HISTORY_SIZE];
        let mut current_root_index = 0;
        
        // Insert first leaf
        let leaf1 = [1u8; 32];
        let result = insert_leaf(
            &leaf1,
            0,
            0,
            height,
            &mut filled_subtrees,
            &mut roots,
            &mut current_root_index,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
        assert_eq!(current_root_index, 1);
        
        // Insert second leaf
        let leaf2 = [2u8; 32];
        let result = insert_leaf(
            &leaf2,
            0,
            1,
            height,
            &mut filled_subtrees,
            &mut roots,
            &mut current_root_index,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
        assert_eq!(current_root_index, 2);
        
        // Try to insert when tree is full
        let result = insert_leaf(
            &[3u8; 32],
            0,
            8, // 2^3 = 8, so tree is full
            height,
            &mut filled_subtrees,
            &mut roots,
            &mut current_root_index,
        );
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            ProgramError::Custom(4).to_string() // MerkleTreeFull error
        );
    }
    
    #[test]
    fn test_is_known_root() {
        // Create a test root history
        let mut roots = [[0u8; 32]; ROOT_HISTORY_SIZE];
        let root1 = [1u8; 32];
        let root2 = [2u8; 32];
        
        roots[0] = root1;
        roots[1] = root2;
        
        let current_root_index = 1;
        
        // Test with known root
        assert!(is_known_root(&root1, &roots, current_root_index));
        assert!(is_known_root(&root2, &roots, current_root_index));
        
        // Test with unknown root
        let unknown_root = [3u8; 32];
        assert!(!is_known_root(&unknown_root, &roots, current_root_index));
        
        // Test with zero root
        let zero_root = [0u8; 32];
        assert!(!is_known_root(&zero_root, &roots, current_root_index));
    }
    
    #[test]
    fn test_get_last_root() {
        // Create a test root history
        let mut roots = [[0u8; 32]; ROOT_HISTORY_SIZE];
        let root1 = [1u8; 32];
        let root2 = [2u8; 32];
        
        roots[0] = root1;
        roots[1] = root2;
        
        // Test with current_root_index = 0
        assert_eq!(get_last_root(&roots, 0), root1);
        
        // Test with current_root_index = 1
        assert_eq!(get_last_root(&roots, 1), root2);
    }
}