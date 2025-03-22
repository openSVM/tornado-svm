//! Tornado Cash Privacy Solution for Solana
//! 
//! This program implements a privacy solution based on zkSNARKs for Solana.
//! It allows users to make private transactions by breaking the on-chain link
//! between sender and recipient addresses.
//!
//! # Architecture
//!
//! The program is organized into several modules:
//!
//! * `error`: Error types for the program
//! * `instruction`: Instruction types and processing
//! * `merkle_tree`: Merkle tree implementation
//! * `processor`: Main program logic
//! * `state`: State types for the program
//! * `utils`: Utility functions
//! * `verifier`: zkSNARK proof verification
//!
//! # Usage
//!
//! The program supports three main instructions:
//!
//! 1. `Initialize`: Initialize a new Tornado instance
//! 2. `Deposit`: Deposit funds into a Tornado instance
//! 3. `Withdraw`: Withdraw funds from a Tornado instance
//!
//! See the [documentation](https://github.com/your-username/tornado-svm/docs) for more details.

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

// Module declarations
pub mod error;
pub mod instruction;
pub mod merkle_tree;
pub mod processor;
pub mod state;
pub mod utils;
pub mod verifier;

// Re-export key types for external use
pub use crate::error::TornadoError;
pub use crate::instruction::TornadoInstruction;
pub use crate::state::{MerkleTree, TornadoInstance};

use crate::processor::Processor;

// Program entrypoint
entrypoint!(process_instruction);

/// Process instruction
///
/// # Arguments
///
/// * `program_id` - The program ID
/// * `accounts` - The accounts required for the instruction
/// * `instruction_data` - The instruction data
///
/// # Returns
///
/// Returns a `ProgramResult` indicating success or failure
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

#[cfg(test)]
mod tests {
    // Unit tests will be added here
}