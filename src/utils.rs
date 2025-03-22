//! Utility functions for the Tornado Cash Privacy Solution

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};

use crate::error::TornadoError;

/// Create a new account with the given size and owner
pub fn create_account<'a>(
    payer: &AccountInfo<'a>,
    new_account: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    space: usize,
    owner: &Pubkey,
    seeds: Option<&[&[u8]]>,
) -> ProgramResult {
    let rent = solana_program::sysvar::rent::Rent::get()?;
    let lamports = rent.minimum_balance(space);

    if seeds.is_some() {
        // Create account with PDA
        let seeds_slice = seeds.unwrap();
        invoke_signed(
            &system_instruction::create_account(
                payer.key,
                new_account.key,
                lamports,
                space as u64,
                owner,
            ),
            &[payer.clone(), new_account.clone(), system_program.clone()],
            &[seeds_slice],
        )?;
    } else {
        // Create account without PDA
        invoke(
            &system_instruction::create_account(
                payer.key,
                new_account.key,
                lamports,
                space as u64,
                owner,
            ),
            &[payer.clone(), new_account.clone(), system_program.clone()],
        )?;
    }

    Ok(())
}

/// Transfer SOL from one account to another
pub fn transfer_sol<'a>(
    from: &AccountInfo<'a>,
    to: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    amount: u64,
    seeds: Option<&[&[u8]]>,
) -> ProgramResult {
    if seeds.is_some() {
        // Transfer with PDA
        let seeds_slice = seeds.unwrap();
        invoke_signed(
            &system_instruction::transfer(from.key, to.key, amount),
            &[from.clone(), to.clone(), system_program.clone()],
            &[seeds_slice],
        )?;
    } else {
        // Transfer without PDA
        invoke(
            &system_instruction::transfer(from.key, to.key, amount),
            &[from.clone(), to.clone(), system_program.clone()],
        )?;
    }

    Ok(())
}

/// Check if a commitment exists in the commitments array
pub fn commitment_exists(commitments: &[[u8; 32]], commitment: &[u8; 32]) -> bool {
    commitments.iter().any(|c| c == commitment)
}

/// Check if a nullifier hash exists in the nullifier_hashes array
pub fn nullifier_hash_exists(nullifier_hashes: &[[u8; 32]], nullifier_hash: &[u8; 32]) -> bool {
    nullifier_hashes.iter().any(|n| n == nullifier_hash)
}

/// Add a commitment to the commitments array
pub fn add_commitment(commitments: &mut Vec<[u8; 32]>, commitment: &[u8; 32]) -> ProgramResult {
    if commitment_exists(commitments, commitment) {
        return Err(TornadoError::CommitmentAlreadyExists.into());
    }
    commitments.push(*commitment);
    Ok(())
}

/// Add a nullifier hash to the nullifier_hashes array
pub fn add_nullifier_hash(nullifier_hashes: &mut Vec<[u8; 32]>, nullifier_hash: &[u8; 32]) -> ProgramResult {
    if nullifier_hash_exists(nullifier_hashes, nullifier_hash) {
        return Err(TornadoError::NullifierAlreadySpent.into());
    }
    nullifier_hashes.push(*nullifier_hash);
    Ok(())
}

/// Compute the Pedersen hash of a nullifier and secret
/// This is a simplified implementation using Keccak256
pub fn compute_commitment(nullifier: &[u8; 32], secret: &[u8; 32]) -> [u8; 32] {
    use sha3::{Digest, Keccak256};
    
    let mut hasher = Keccak256::new();
    hasher.update(nullifier);
    hasher.update(secret);
    let result = hasher.finalize();
    
    let mut commitment = [0u8; 32];
    commitment.copy_from_slice(&result[..32]);
    
    commitment
}

/// Compute the hash of a nullifier
/// This is a simplified implementation using Keccak256
pub fn compute_nullifier_hash(nullifier: &[u8; 32]) -> [u8; 32] {
    use sha3::{Digest, Keccak256};
    
    let mut hasher = Keccak256::new();
    hasher.update(nullifier);
    let result = hasher.finalize();
    
    let mut nullifier_hash = [0u8; 32];
    nullifier_hash.copy_from_slice(&result[..32]);
    
    nullifier_hash
}