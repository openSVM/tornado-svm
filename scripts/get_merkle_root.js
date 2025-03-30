#!/usr/bin/env node

const { Connection, PublicKey } = require('@solana/web3.js');
const borsh = require('borsh');

// Check command line arguments
if (process.argv.length < 4) {
  console.error('Usage: node get_merkle_root.js <merkle_tree_pubkey> <rpc_url>');
  process.exit(1);
}

const merkleTreePubkey = process.argv[2];
const rpcUrl = process.argv[3];

// Define the MerkleTree class schema for Borsh deserialization
class MerkleTree {
  constructor(properties) {
    Object.assign(this, properties);
  }
}

// Define the schema for Borsh deserialization
const schema = new Map([
  [
    MerkleTree,
    {
      kind: 'struct',
      fields: [
        ['is_initialized', 'u8'],
        ['height', 'u8'],
        ['current_index', 'u32'],
        ['next_index', 'u32'],
        ['current_root_index', 'u8'], // Changed from 'u32' to 'u8' to match the Solana program
        ['roots', [['u8', 32], 30]], // Array of 30 roots, each 32 bytes
        ['filled_subtrees', [['u8', 32]]], // Variable length array of 32-byte arrays
        ['nullifier_hashes', [['u8', 32]]], // Variable length array of 32-byte arrays
        ['commitments', [['u8', 32]]], // Variable length array of 32-byte arrays
      ],
    },
  ],
]);

async function getMerkleRoot() {
  try {
    // Connect to the Solana network
    const connection = new Connection(rpcUrl, 'confirmed');

    // Get the account data
    const accountInfo = await connection.getAccountInfo(new PublicKey(merkleTreePubkey));
    
    if (!accountInfo) {
      console.error('Merkle tree account not found');
      process.exit(1);
    }

    // Deserialize the account data
    const merkleTree = borsh.deserialize(schema, MerkleTree, accountInfo.data);

    // Get the current root
    const currentRootIndex = merkleTree.current_root_index;
    const currentRoot = merkleTree.roots[currentRootIndex];

    // Convert the root to hex string
    const rootHex = Buffer.from(currentRoot).toString('hex');
    
    // Output the root
    console.log(rootHex);
  } catch (error) {
    console.error('Error:', error);
    process.exit(1);
  }
}

getMerkleRoot();