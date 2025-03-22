//! Program processor for the Tornado Cash Privacy Solution
//!
//! This module contains the main logic for processing instructions for the
//! Tornado Cash Privacy Solution for Solana. It handles initialization,
//! deposits, and withdrawals.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    system_program,
};

use crate::{
    error::TornadoError,
    instruction::TornadoInstruction,
    merkle_tree::{insert_leaf, is_known_root},
    state::{MerkleTree, TornadoInstance, ROOT_HISTORY_SIZE},
    utils::{add_commitment, add_nullifier_hash, commitment_exists, create_account, nullifier_hash_exists, transfer_sol},
    verifier::verify_tornado_proof,
};

/// Program processor
pub struct Processor;

impl Processor {
    /// Process a Tornado Cash instruction
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
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        // Deserialize the instruction data
        let instruction = TornadoInstruction::try_from_slice(instruction_data)
            .map_err(|_| TornadoError::InvalidInstructionData)?;

        // Process the instruction
        match instruction {
            TornadoInstruction::Initialize {
                denomination,
                merkle_tree_height,
            } => {
                msg!("Instruction: Initialize");
                Self::process_initialize(program_id, accounts, denomination, merkle_tree_height)
            }
            TornadoInstruction::Deposit { commitment } => {
                msg!("Instruction: Deposit");
                Self::process_deposit(program_id, accounts, &commitment)
            }
            TornadoInstruction::Withdraw {
                proof,
                root,
                nullifier_hash,
                recipient,
                relayer,
                fee,
                refund,
            } => {
                msg!("Instruction: Withdraw");
                Self::process_withdraw(
                    program_id,
                    accounts,
                    &proof,
                    &root,
                    &nullifier_hash,
                    &recipient,
                    &relayer,
                    fee,
                    refund,
                )
            }
        }
    }

    /// Process an Initialize instruction
    ///
    /// # Arguments
    ///
    /// * `program_id` - The program ID
    /// * `accounts` - The accounts required for the instruction
    /// * `denomination` - The denomination amount for this instance
    /// * `merkle_tree_height` - The height of the Merkle tree
    ///
    /// # Returns
    ///
    /// Returns a `ProgramResult` indicating success or failure
    fn process_initialize(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        denomination: u64,
        merkle_tree_height: u8,
    ) -> ProgramResult {
        // Get the account information
        let account_info_iter = &mut accounts.iter();
        let payer = next_account_info(account_info_iter)?;
        let tornado_instance_info = next_account_info(account_info_iter)?;
        let system_program_info = next_account_info(account_info_iter)?;

        // Check if the tornado instance account is already initialized
        if !tornado_instance_info.data.borrow().iter().all(|&x| x == 0) {
            return Err(TornadoError::AccountAlreadyInitialized.into());
        }

        // Create a new Merkle tree account
        let merkle_tree_seed = &[
            b"merkle_tree",
            tornado_instance_info.key.as_ref(),
            &[0],
        ];
        let (merkle_tree_key, _) =
            Pubkey::find_program_address(merkle_tree_seed, program_id);

        // Create a new verifier account
        let verifier_seed = &[
            b"verifier",
            tornado_instance_info.key.as_ref(),
            &[0],
        ];
        let (verifier_key, _) =
            Pubkey::find_program_address(verifier_seed, program_id);

        // Initialize the tornado instance
        let tornado_instance = TornadoInstance {
            is_initialized: true,
            denomination,
            merkle_tree_height,
            merkle_tree: merkle_tree_key,
            verifier: verifier_key,
        };

        // Save the tornado instance
        tornado_instance.serialize(&mut *tornado_instance_info.data.borrow_mut())?;

        msg!("Tornado instance initialized with denomination {} and height {}", denomination, merkle_tree_height);
        Ok(())
    }

    /// Process a Deposit instruction
    ///
    /// # Arguments
    ///
    /// * `program_id` - The program ID
    /// * `accounts` - The accounts required for the instruction
    /// * `commitment` - The commitment to deposit
    ///
    /// # Returns
    ///
    /// Returns a `ProgramResult` indicating success or failure
    fn process_deposit(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        commitment: &[u8; 32],
    ) -> ProgramResult {
        // Get the account information
        let account_info_iter = &mut accounts.iter();
        let payer = next_account_info(account_info_iter)?;
        let tornado_instance_info = next_account_info(account_info_iter)?;
        let merkle_tree_info = next_account_info(account_info_iter)?;
        let system_program_info = next_account_info(account_info_iter)?;

        // Check if the tornado instance is initialized
        let tornado_instance = TornadoInstance::unpack(&tornado_instance_info.data.borrow())?;
        if !tornado_instance.is_initialized {
            return Err(TornadoError::AccountNotInitialized.into());
        }

        // Check if the merkle tree account is the correct one
        if tornado_instance.merkle_tree != *merkle_tree_info.key {
            return Err(TornadoError::InvalidAccountData.into());
        }

        // Check if the commitment already exists
        let mut merkle_tree = MerkleTree::try_from_slice(&merkle_tree_info.data.borrow())?;
        if commitment_exists(&merkle_tree.commitments, commitment) {
            return Err(TornadoError::CommitmentAlreadyExists.into());
        }

        // Transfer the denomination amount from the payer to the tornado instance
        transfer_sol(
            payer,
            tornado_instance_info,
            system_program_info,
            tornado_instance.denomination,
            None,
        )?;

        // Insert the commitment into the Merkle tree
        let inserted_index = insert_leaf(
            commitment,
            merkle_tree.current_index,
            merkle_tree.next_index,
            merkle_tree.height,
            &mut merkle_tree.filled_subtrees,
            &mut merkle_tree.roots,
            &mut merkle_tree.current_root_index,
        )?;

        // Update the Merkle tree state
        merkle_tree.next_index += 1;

        // Add the commitment to the commitments array
        add_commitment(&mut merkle_tree.commitments, commitment)?;

        // Save the updated Merkle tree
        merkle_tree.serialize(&mut *merkle_tree_info.data.borrow_mut())?;

        msg!("Deposit successful. Leaf index: {}", inserted_index);

        Ok(())
    }

    /// Process a Withdraw instruction
    ///
    /// # Arguments
    ///
    /// * `program_id` - The program ID
    /// * `accounts` - The accounts required for the instruction
    /// * `proof` - The zkSNARK proof
    /// * `root` - The Merkle root
    /// * `nullifier_hash` - The nullifier hash
    /// * `recipient_pubkey` - The recipient public key
    /// * `relayer_pubkey` - The relayer public key
    /// * `fee` - The fee to pay to the relayer
    /// * `refund` - The refund amount (for token instances)
    ///
    /// # Returns
    ///
    /// Returns a `ProgramResult` indicating success or failure
    fn process_withdraw(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        proof: &[u8],
        root: &[u8; 32],
        nullifier_hash: &[u8; 32],
        recipient_pubkey: &Pubkey,
        relayer_pubkey: &Pubkey,
        fee: u64,
        refund: u64,
    ) -> ProgramResult {
        // Get the account information
        let account_info_iter = &mut accounts.iter();
        let payer = next_account_info(account_info_iter)?;
        let tornado_instance_info = next_account_info(account_info_iter)?;
        let merkle_tree_info = next_account_info(account_info_iter)?;
        let recipient_info = next_account_info(account_info_iter)?;
        let relayer_info = next_account_info(account_info_iter)?;
        let system_program_info = next_account_info(account_info_iter)?;

        // Check if the tornado instance is initialized
        let tornado_instance = TornadoInstance::unpack(&tornado_instance_info.data.borrow())?;
        if !tornado_instance.is_initialized {
            return Err(TornadoError::AccountNotInitialized.into());
        }

        // Check if the merkle tree account is the correct one
        if tornado_instance.merkle_tree != *merkle_tree_info.key {
            return Err(TornadoError::InvalidAccountData.into());
        }

        // Check if the recipient account is the correct one
        if recipient_pubkey != recipient_info.key {
            return Err(TornadoError::InvalidRecipient.into());
        }

        // Check if the relayer account is the correct one
        if relayer_pubkey != relayer_info.key {
            return Err(TornadoError::InvalidRelayer.into());
        }

        // Check if the fee is valid
        if fee > tornado_instance.denomination {
            return Err(TornadoError::InvalidFee.into());
        }

        // Check if the refund is valid (should be 0 for SOL)
        if refund != 0 {
            return Err(TornadoError::InvalidAmount.into());
        }

        // Check if the nullifier hash has already been spent
        let mut merkle_tree = MerkleTree::try_from_slice(&merkle_tree_info.data.borrow())?;
        if nullifier_hash_exists(&merkle_tree.nullifier_hashes, nullifier_hash) {
            return Err(TornadoError::NullifierAlreadySpent.into());
        }

        // Check if the root is known
        if !is_known_root(root, &merkle_tree.roots, merkle_tree.current_root_index) {
            return Err(TornadoError::InvalidMerkleRoot.into());
        }

        // Prepare the public inputs for the proof verification
        let mut public_inputs = [0u8; 192]; // 6 public inputs * 32 bytes
        public_inputs[0..32].copy_from_slice(root);
        public_inputs[32..64].copy_from_slice(nullifier_hash);
        public_inputs[64..96].copy_from_slice(&recipient_pubkey.to_bytes());
        public_inputs[96..128].copy_from_slice(&relayer_pubkey.to_bytes());
        public_inputs[128..160].copy_from_slice(&fee.to_le_bytes());
        public_inputs[160..192].copy_from_slice(&refund.to_le_bytes());

        // Verify the proof
        if !verify_tornado_proof(proof, &public_inputs)? {
            return Err(TornadoError::InvalidProof.into());
        }

        // Add the nullifier hash to the nullifier_hashes array
        add_nullifier_hash(&mut merkle_tree.nullifier_hashes, nullifier_hash)?;

        // Transfer the denomination amount minus the fee to the recipient
        transfer_sol(
            tornado_instance_info,
            recipient_info,
            system_program_info,
            tornado_instance.denomination - fee,
            None,
        )?;

        // If there's a fee, transfer it to the relayer
        if fee > 0 {
            transfer_sol(
                tornado_instance_info,
                relayer_info,
                system_program_info,
                fee,
                None,
            )?;
        }

        // Save the updated Merkle tree
        merkle_tree.serialize(&mut *merkle_tree_info.data.borrow_mut())?;

        msg!("Withdrawal successful");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // Unit tests will be added here
}