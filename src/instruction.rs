//! Instruction types for the Tornado Cash Privacy Solution

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program,
};

/// Instructions supported by the Tornado Cash program
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq)]
pub enum TornadoInstruction {
    /// Initialize a new Tornado instance
    ///
    /// Accounts expected:
    /// 0. `[signer]` The account that will pay for the initialization
    /// 1. `[writable]` The Tornado instance account to initialize
    /// 2. `[]` System program
    Initialize {
        /// The denomination amount for this instance
        denomination: u64,
        /// The height of the Merkle tree
        merkle_tree_height: u8,
    },

    /// Deposit funds into the Tornado instance
    ///
    /// Accounts expected:
    /// 0. `[signer]` The account that will deposit funds
    /// 1. `[writable]` The Tornado instance account
    /// 2. `[writable]` The Merkle tree account
    /// 3. `[]` System program
    Deposit {
        /// The commitment to deposit
        commitment: [u8; 32],
    },

    /// Withdraw funds from the Tornado instance
    ///
    /// Accounts expected:
    /// 0. `[signer]` The account that will pay for the transaction (can be the relayer)
    /// 1. `[writable]` The Tornado instance account
    /// 2. `[writable]` The Merkle tree account
    /// 3. `[writable]` The recipient account
    /// 4. `[writable, optional]` The relayer account
    /// 5. `[]` System program
    Withdraw {
        /// The proof data
        proof: Vec<u8>,
        /// The Merkle root
        root: [u8; 32],
        /// The nullifier hash
        nullifier_hash: [u8; 32],
        /// The recipient address
        recipient: Pubkey,
        /// The relayer address
        relayer: Pubkey,
        /// The fee to pay to the relayer
        fee: u64,
        /// The refund amount (for token instances)
        refund: u64,
    },
}

/// Create an Initialize instruction
pub fn initialize(
    program_id: &Pubkey,
    payer: &Pubkey,
    tornado_instance: &Pubkey,
    denomination: u64,
    merkle_tree_height: u8,
) -> Result<Instruction, ProgramError> {
    let data = TornadoInstruction::Initialize {
        denomination,
        merkle_tree_height,
    }
    .try_to_vec()?;

    let accounts = vec![
        AccountMeta::new(*payer, true),
        AccountMeta::new(*tornado_instance, false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Create a Deposit instruction
pub fn deposit(
    program_id: &Pubkey,
    payer: &Pubkey,
    tornado_instance: &Pubkey,
    merkle_tree: &Pubkey,
    commitment: [u8; 32],
) -> Result<Instruction, ProgramError> {
    let data = TornadoInstruction::Deposit { commitment }.try_to_vec()?;

    let accounts = vec![
        AccountMeta::new(*payer, true),
        AccountMeta::new(*tornado_instance, false),
        AccountMeta::new(*merkle_tree, false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Create a Withdraw instruction
pub fn withdraw(
    program_id: &Pubkey,
    payer: &Pubkey,
    tornado_instance: &Pubkey,
    merkle_tree: &Pubkey,
    recipient: &Pubkey,
    relayer: &Pubkey,
    proof: Vec<u8>,
    root: [u8; 32],
    nullifier_hash: [u8; 32],
    fee: u64,
    refund: u64,
) -> Result<Instruction, ProgramError> {
    let data = TornadoInstruction::Withdraw {
        proof,
        root,
        nullifier_hash,
        recipient: *recipient,
        relayer: *relayer,
        fee,
        refund,
    }
    .try_to_vec()?;

    let accounts = vec![
        AccountMeta::new(*payer, true),
        AccountMeta::new(*tornado_instance, false),
        AccountMeta::new(*merkle_tree, false),
        AccountMeta::new(*recipient, false),
        AccountMeta::new(*relayer, false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}