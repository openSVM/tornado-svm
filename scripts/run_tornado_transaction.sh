#!/bin/bash
set -e

# If SOLANA_PATH is set, add it to PATH
if [ -n "$SOLANA_PATH" ]; then
    echo "Adding Solana binaries to PATH: $SOLANA_PATH"
    export PATH="$SOLANA_PATH:$PATH"
fi

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'  # Add this line
NC='\033[0m' # No Color

echo -e "${GREEN}Starting Tornado Cash Privacy Solution Transaction Script${NC}"

# Check if solana is installed and print more debugging information if not found
if ! command -v solana &> /dev/null; then
    echo "Error: Solana CLI is not installed or not in PATH."
    echo "Current PATH: $PATH"
    echo "Please install Solana CLI or ensure it's in your PATH."
    
    # Check for common Solana CLI locations
    for dir in "$HOME/.local/share/solana/install/active_release/bin" "/usr/local/bin" "/usr/bin"; do
        if [ -f "$dir/solana" ]; then
            echo "Found solana in $dir but it's not in PATH. Try adding: export PATH=\"$dir:\$PATH\""
        fi
    done
    
    exit 1
fi

# Print Solana version for debugging
echo "Using Solana version: $(solana --version)"

# Check if the tornado-cli.js exists
if [ ! -f "../client/tornado-cli.js" ]; then
    echo "Error: tornado-cli.js not found. Make sure you're running this script from the scripts directory."
    exit 1
fi

# Configuration
PROGRAM_ID=""
TORNADO_INSTANCE=""
MERKLE_TREE=""
WALLET_PATH="$HOME/.config/solana/id.json"
DENOMINATION=1 # 1 SOL
MERKLE_TREE_HEIGHT=20
RPC_URL="https://api.testnet.solana.com"

# Step 1: Configure Solana CLI to use testnet
echo -e "${YELLOW}Step 1: Configuring Solana CLI to use testnet...${NC}"
solana config set --url $RPC_URL
echo "Connected to Solana testnet"

# Create a new wallet if it doesn't exist
if [ ! -f "$WALLET_PATH" ]; then
    echo "Creating new wallet..."
    solana-keygen new --no-bip39-passphrase -o "$WALLET_PATH"
fi

# Airdrop SOL to the wallet (testnet has a lower limit)
echo "Airdropping 1 SOL to wallet..."
solana airdrop 1 $(solana address) || true
sleep 2

# Step 2: Install dependencies for the client
echo -e "${YELLOW}Step 2: Installing dependencies...${NC}"
cd ../client
npm install @solana/web3.js commander fs crypto bn.js bs58 borsh
cd ../scripts

# Step 3: Build and deploy the program
echo -e "${YELLOW}Step 3: Building and deploying the program...${NC}"
cd ..
echo "Building the program..."

# Print Rust and cargo info for debugging
echo "Rust version: $(rustc --version)"
echo "Cargo version: $(cargo --version)"

# Check for Solana BPF tools
echo "Checking for Solana BPF/SBF tools..."
for cmd in "cargo build-sbf" "cargo build-bpf"; do
    if command -v $cmd &> /dev/null; then
        echo "Found $cmd"
    fi
done

# Try the newer cargo build-sbf command first, fall back to cargo build-bpf if not available
if command -v cargo build-sbf &> /dev/null; then
    echo "Using cargo build-sbf..."
    cargo build-sbf || { echo -e "${RED}Error: Failed to build the program.${NC}"; exit 1; }
else
    echo "Using cargo build-bpf..."
    cargo build-bpf || { echo -e "${RED}Error: Failed to build the program.${NC}"; exit 1; }
fi

echo "Deploying the program..."
echo "Using solana from: $(which solana)"
DEPLOY_OUTPUT=$(solana program deploy target/deploy/tornado_svm.so)
PROGRAM_ID=$(echo "$DEPLOY_OUTPUT" | grep "Program Id:" | awk '{print $3}')

if [ -z "$PROGRAM_ID" ]; then
    echo -e "${RED}Error: Failed to deploy the program.${NC}"
    echo "$DEPLOY_OUTPUT"
    exit 1
fi

echo "Program deployed with ID: $PROGRAM_ID"

# Update the program ID in the tornado-cli.js
sed -i "s/YourProgramIdHere/$PROGRAM_ID/g" client/tornado-cli.js

# Step 4: Initialize a tornado instance
echo -e "${YELLOW}Step 4: Initializing tornado instance...${NC}"
cd client
INIT_OUTPUT=$(npx ./tornado-cli.js initialize --keypair "$WALLET_PATH" --denomination $DENOMINATION --height $MERKLE_TREE_HEIGHT)
TORNADO_INSTANCE=$(echo "$INIT_OUTPUT" | grep "Tornado instance created:" | awk '{print $4}')

if [ -z "$TORNADO_INSTANCE" ]; then
    echo -e "${RED}Error: Failed to initialize tornado instance.${NC}"
    echo "$INIT_OUTPUT"
    exit 1
fi

echo "Tornado instance created: $TORNADO_INSTANCE"

# Wait for the transaction to be confirmed
sleep 5

# Step 5: Generate a commitment
echo -e "${YELLOW}Step 5: Generating commitment...${NC}"
COMMITMENT_OUTPUT=$(npx ./tornado-cli.js generate-commitment)
NOTE_PATH=$(echo "$COMMITMENT_OUTPUT" | grep "Note saved to" | awk '{print $4}')
COMMITMENT=$(echo "$COMMITMENT_OUTPUT" | grep "Commitment:" | awk '{print $2}')

if [ -z "$NOTE_PATH" ] || [ -z "$COMMITMENT" ]; then
    echo -e "${RED}Error: Failed to generate commitment.${NC}"
    echo "$COMMITMENT_OUTPUT"
    exit 1
fi

echo "Note saved to: $NOTE_PATH"
echo "Commitment: $COMMITMENT"

# Step 6: Deposit funds
echo -e "${YELLOW}Step 6: Depositing funds...${NC}"
DEPOSIT_OUTPUT=$(npx ./tornado-cli.js deposit --keypair "$WALLET_PATH" --instance "$TORNADO_INSTANCE" --commitment "$COMMITMENT" --amount $DENOMINATION)
DEPOSIT_SIGNATURE=$(echo "$DEPOSIT_OUTPUT" | grep "Transaction signature:" | awk '{print $3}')

if [ -z "$DEPOSIT_SIGNATURE" ]; then
    echo -e "${RED}Error: Failed to deposit funds.${NC}"
    echo "$DEPOSIT_OUTPUT"
    exit 1
fi

echo "Deposit transaction signature: $DEPOSIT_SIGNATURE"

# Wait for the transaction to be confirmed
echo "Waiting for deposit to be confirmed..."
sleep 10

# Step 7: Get the Merkle tree account
echo -e "${YELLOW}Step 7: Getting Merkle tree account...${NC}"
# Get the Merkle tree account using find-program-address
MERKLE_TREE=$(solana address find-program-address \
    --input "merkle_tree" \
    --input "$TORNADO_INSTANCE" \
    --input "0" \
    --program-id "$PROGRAM_ID" | head -1)

if [ -z "$MERKLE_TREE" ]; then
    echo -e "${RED}Error: Failed to get Merkle tree account.${NC}"
    # Try alternative method for older Solana CLI versions
    echo "Trying alternative method..."
    # In older versions, we need to use a different approach
    # We'll use the tornado-cli.js to get the Merkle tree account
    cd ../client
    MERKLE_TREE_OUTPUT=$(node -e "
        const { PublicKey } = require('@solana/web3.js');
        const programId = new PublicKey('$PROGRAM_ID');
        const tornadoInstance = new PublicKey('$TORNADO_INSTANCE');
        const seeds = [
            Buffer.from('merkle_tree', 'utf8'),
            tornadoInstance.toBuffer(),
            Buffer.from([0])
        ];
        const [merkleTreePubkey] = PublicKey.findProgramAddressSync(seeds, programId);
        console.log(merkleTreePubkey.toString());
    ")
    MERKLE_TREE=$MERKLE_TREE_OUTPUT
    cd ../scripts
    
    if [ -z "$MERKLE_TREE" ]; then
        echo -e "${RED}Error: Failed to get Merkle tree account using alternative method.${NC}"
        exit 1
    fi
fi

echo "Merkle tree account: $MERKLE_TREE"

# Step 8: Get the Merkle root
echo -e "${YELLOW}Step 8: Getting Merkle root...${NC}"
cd ../scripts
ROOT=$(node get_merkle_root.js "$MERKLE_TREE" "$RPC_URL")

if [ -z "$ROOT" ]; then
    echo -e "${RED}Error: Failed to get Merkle root.${NC}"
    # Fallback to a dummy root for testing
    ROOT="0000000000000000000000000000000000000000000000000000000000000000"
    echo "Using fallback root: $ROOT"
else
    echo "Merkle root: $ROOT"
fi

# Step 9: Generate a proof for withdrawal
echo -e "${YELLOW}Step 9: Generating proof for withdrawal...${NC}"
cd ../client
RECIPIENT=$(solana address)
PROOF_OUTPUT=$(npx ./tornado-cli.js generate-proof --note "$NOTE_PATH" --root "$ROOT" --recipient "$RECIPIENT")
PROOF=$(echo "$PROOF_OUTPUT" | grep "Proof:" | awk '{print $2}')
NULLIFIER_HASH=$(echo "$PROOF_OUTPUT" | grep "Nullifier hash:" | awk '{print $3}')

if [ -z "$PROOF" ] || [ -z "$NULLIFIER_HASH" ]; then
    echo -e "${RED}Error: Failed to generate proof.${NC}"
    echo "$PROOF_OUTPUT"
    exit 1
fi

echo "Proof: $PROOF"
echo "Nullifier hash: $NULLIFIER_HASH"

# Step 10: Withdraw funds
echo -e "${YELLOW}Step 10: Withdrawing funds...${NC}"
WITHDRAW_OUTPUT=$(npx ./tornado-cli.js withdraw --keypair "$WALLET_PATH" --instance "$TORNADO_INSTANCE" --proof "$PROOF" --root "$ROOT" --nullifier-hash "$NULLIFIER_HASH" --recipient "$RECIPIENT")
WITHDRAW_SIGNATURE=$(echo "$WITHDRAW_OUTPUT" | grep "Transaction signature:" | awk '{print $3}')

if [ -z "$WITHDRAW_SIGNATURE" ]; then
    echo -e "${RED}Error: Failed to withdraw funds.${NC}"
    echo "$WITHDRAW_OUTPUT"
    exit 1
fi

echo "Withdraw transaction signature: $WITHDRAW_SIGNATURE"

# Wait for the transaction to be confirmed
echo "Waiting for withdrawal to be confirmed..."
sleep 10

echo -e "${GREEN}Transaction completed successfully!${NC}"

# Check recipient balance
RECIPIENT_BALANCE=$(solana balance $RECIPIENT)
echo "Recipient balance: $RECIPIENT_BALANCE SOL"

# Cleanup
echo -e "${YELLOW}Cleaning up...${NC}"
echo "No cleanup needed for testnet"

echo -e "${GREEN}Script completed!${NC}"
