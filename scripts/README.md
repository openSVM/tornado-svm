# Tornado Cash Privacy Solution Scripts

This directory contains scripts to help you interact with the Tornado Cash Privacy Solution on Solana.

## Prerequisites

Before running the scripts, make sure you have the following installed:

- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Node.js](https://nodejs.org/) (v14 or later)
- [npm](https://www.npmjs.com/)

## Scripts

### run_tornado_transaction.sh

This script performs a complete transaction through the Tornado Cash Privacy Solution on a local validator. It:

1. Starts a local validator
2. Builds and deploys the Tornado Cash program
3. Initializes a Tornado instance
4. Generates a commitment
5. Deposits funds into the Tornado instance
6. Gets the Merkle root
7. Generates a proof for withdrawal
8. Withdraws funds from the Tornado instance

#### Usage

```bash
cd scripts
chmod +x run_tornado_transaction.sh
./run_tornado_transaction.sh
```

### get_merkle_root.js

This script queries the Merkle tree account and extracts the current root.

#### Usage

```bash
node get_merkle_root.js <merkle_tree_pubkey> <rpc_url>
```

#### Parameters

- `merkle_tree_pubkey`: The public key of the Merkle tree account
- `rpc_url`: The RPC URL of the Solana network (e.g., http://localhost:8899)

## Troubleshooting

If you encounter any issues:

1. Make sure you have the latest version of Solana CLI and Node.js installed
2. Check that you have sufficient SOL in your wallet
3. Ensure the local validator is running
4. Check the logs for any error messages

For more detailed information, refer to the main README.md file in the root directory.