//! Tornado Cash Privacy Solution for Solana
//! 
//! This program implements a privacy solution based on zkSNARKs for Solana.
//! It allows users to make private transactions by breaking the on-chain link
//! between sender and recipient addresses.

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

pub mod error;
pub mod instruction;
pub mod merkle_tree;
pub mod processor;
pub mod state;
pub mod utils;
pub mod verifier;

use crate::processor::Processor;

// Program entrypoint
entrypoint!(process_instruction);

/// Process instruction
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Tornado Cash Privacy Solution for Solana");
    
    if let Err(error) = Processor::process(program_id, accounts, instruction_data) {
        // Program errors
        msg!("Error: {:?}", error);
        return Err(error);
    }
    
    Ok(())
}