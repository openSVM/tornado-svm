//! Tests for the Tornado Cash Privacy Solution

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    hash::Hash,
    program_pack::Pack,
    pubkey::Pubkey,
    system_instruction,
};
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

use tornado_svm::{
    instruction::{deposit, initialize, withdraw},
    state::{MerkleTree, TornadoInstance},
    utils::{compute_commitment, compute_nullifier_hash},
};

#[tokio::test]
async fn test_tornado_flow() {
    // Create program test environment
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "tornado_svm",
        program_id,
        processor!(tornado_svm::process_instruction),
    );

    // Create accounts
    let payer = Keypair::new();
    let tornado_instance = Keypair::new();
    let merkle_tree = Keypair::new();
    let recipient = Keypair::new();
    let relayer = Keypair::new();

    // Add accounts to the test environment
    program_test.add_account(
        payer.pubkey(),
        Account {
            lamports: 1_000_000_000,
            data: vec![],
            owner: solana_program::system_program::id(),
            executable: false,
            rent_epoch: 0,
        },
    );

    // Start the test environment
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Initialize the Tornado instance
    let denomination = 100_000_000; // 1 SOL
    let merkle_tree_height = 20;

    let initialize_ix = initialize(
        &program_id,
        &payer.pubkey(),
        &tornado_instance.pubkey(),
        denomination,
        merkle_tree_height,
    )
    .unwrap();

    let mut transaction = Transaction::new_with_payer(
        &[initialize_ix],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &tornado_instance], recent_blockhash);

    banks_client.process_transaction(transaction).await.unwrap();

    // Generate a nullifier and secret
    let nullifier = [1u8; 32];
    let secret = [2u8; 32];

    // Compute the commitment
    let commitment = compute_commitment(&nullifier, &secret);

    // Deposit
    let deposit_ix = deposit(
        &program_id,
        &payer.pubkey(),
        &tornado_instance.pubkey(),
        &merkle_tree.pubkey(),
        commitment,
    )
    .unwrap();

    let mut transaction = Transaction::new_with_payer(
        &[deposit_ix],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);

    banks_client.process_transaction(transaction).await.unwrap();

    // Compute the nullifier hash
    let nullifier_hash = compute_nullifier_hash(&nullifier);

    // Get the Merkle root
    let merkle_tree_account = banks_client
        .get_account(merkle_tree.pubkey())
        .await
        .unwrap()
        .unwrap();
    let merkle_tree_data = MerkleTree::try_from_slice(&merkle_tree_account.data).unwrap();
    let root = merkle_tree_data.roots[merkle_tree_data.current_root_index as usize];

    // Generate a dummy proof (in a real scenario, this would be a valid zkSNARK proof)
    let proof = vec![0u8; 256];

    // Withdraw
    let withdraw_ix = withdraw(
        &program_id,
        &payer.pubkey(),
        &tornado_instance.pubkey(),
        &merkle_tree.pubkey(),
        &recipient.pubkey(),
        &relayer.pubkey(),
        proof,
        root,
        nullifier_hash,
        0, // No fee
        0, // No refund
    )
    .unwrap();

    let mut transaction = Transaction::new_with_payer(
        &[withdraw_ix],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);

    banks_client.process_transaction(transaction).await.unwrap();

    // Verify that the recipient received the funds
    let recipient_account = banks_client
        .get_account(recipient.pubkey())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(recipient_account.lamports, denomination);

    // Verify that the nullifier hash is marked as spent
    let merkle_tree_account = banks_client
        .get_account(merkle_tree.pubkey())
        .await
        .unwrap()
        .unwrap();
    let merkle_tree_data = MerkleTree::try_from_slice(&merkle_tree_account.data).unwrap();
    assert!(merkle_tree_data.nullifier_hashes.contains(&nullifier_hash));
}