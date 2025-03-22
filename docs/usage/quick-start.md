# Quick Start Guide

This guide will help you get started with the Tornado Cash Privacy Solution for Solana. It covers the basic steps to make a private transaction.

## Prerequisites

Before you begin, make sure you have:

1. Node.js (v14 or later) installed
2. Solana CLI (v1.16.0 or later) installed
3. A Solana wallet with some SOL for transactions

## Installation

Install the Tornado Cash CLI tool:

```bash
# Clone the repository
git clone https://github.com/your-username/tornado-svm.git
cd tornado-svm

# Install dependencies
npm install

# Build the Solana program
cargo build-bpf

# Deploy the program
solana program deploy target/deploy/tornado_svm.so

# Install the CLI tool
cd client
npm install
```

## Basic Usage

### Generate a Commitment

The first step is to generate a commitment, which is a hash of a nullifier and a secret. This commitment will be stored in the Merkle tree when you make a deposit.

```bash
# Generate a commitment
npx tornado-cli generate-commitment
```

This will output something like:

```
Note saved to ./note-1647123456789.json
Commitment: 2fe54c60d3acab...
```

Keep the note file safe! You'll need it to withdraw your funds later.

### Initialize a Tornado Instance

Before you can make deposits, you need to initialize a Tornado instance with a specific denomination and Merkle tree height.

```bash
# Initialize a Tornado instance with 1 SOL denomination and height 20
npx tornado-cli initialize --denomination 1000000000 --height 20
```

This will output something like:

```
Transaction signature: 4Qv4hVfABQzrZSCnTgq9...
Tornado instance created: 8ZJW1zXrNcKazM9E...
```

Save the Tornado instance address for later use.

### Deposit Funds

Now you can deposit funds into the Tornado instance using the commitment you generated earlier.

```bash
# Deposit 1 SOL
npx tornado-cli deposit --instance 8ZJW1zXrNcKazM9E... --commitment 2fe54c60d3acab... --amount 1
```

This will output something like:

```
Transaction signature: 5Rv5gTfBCQzrZSCnTgq9...
Deposit successful!
```

### Wait for Confirmation

Wait for the transaction to be confirmed on the Solana blockchain. This usually takes a few seconds.

### Generate a Proof

When you're ready to withdraw your funds, you need to generate a proof that you know the nullifier and secret corresponding to a commitment in the Merkle tree.

First, you need to get the current Merkle root. You can use the Solana CLI to fetch the Merkle tree account data and extract the root.

```bash
# Generate a proof
npx tornado-cli generate-proof --note ./note-1647123456789.json --root 3fe54c60d3acab... --recipient 9ZJW1zXrNcKazM9E...
```

This will output something like:

```
Proof: 000000000000000000...
Nullifier hash: 4fe54c60d3acab...
```

### Withdraw Funds

Finally, you can withdraw your funds to a different address.

```bash
# Withdraw funds
npx tornado-cli withdraw --instance 8ZJW1zXrNcKazM9E... --proof 000000000000000000... --root 3fe54c60d3acab... --nullifier-hash 4fe54c60d3acab... --recipient 9ZJW1zXrNcKazM9E...
```

This will output something like:

```
Transaction signature: 6Sv5gTfBCQzrZSCnTgq9...
Withdrawal successful!
```

## Next Steps

Now that you've completed a basic private transaction, you can explore more advanced features:

1. [Using relayers](advanced/relayers.md) to pay for gas fees
2. [Multiple denominations](advanced/denominations.md) for different transaction sizes
3. [Custom Merkle tree heights](advanced/merkle-tree-heights.md) for different security levels

For a complete reference of the CLI commands, see the [CLI Reference](cli/index.md).