#!/usr/bin/env node

const { Connection, PublicKey, Keypair, Transaction, SystemProgram, sendAndConfirmTransaction } = require('@solana/web3.js');
const { program } = require('commander');
const fs = require('fs');
const crypto = require('crypto');
const BN = require('bn.js');
const bs58 = require('bs58');

// Constants
const PROGRAM_ID = new PublicKey('YourProgramIdHere'); // Replace with your deployed program ID
const DEFAULT_DENOMINATION = 1_000_000_000; // 1 SOL
const DEFAULT_MERKLE_TREE_HEIGHT = 20;

// Connect to Solana network
const connection = new Connection('https://api.devnet.solana.com', 'confirmed');

// Load wallet from keypair file
function loadWallet(keypairPath) {
  const keypairData = JSON.parse(fs.readFileSync(keypairPath, 'utf8'));
  return Keypair.fromSecretKey(new Uint8Array(keypairData));
}

// Generate a random commitment
function generateCommitment() {
  const nullifier = crypto.randomBytes(32);
  const secret = crypto.randomBytes(32);
  
  // Compute commitment (in a real implementation, this would use Pedersen hash)
  const commitment = crypto.createHash('sha256')
    .update(Buffer.concat([nullifier, secret]))
    .digest();
  
  // Save the note
  const note = {
    nullifier: nullifier.toString('hex'),
    secret: secret.toString('hex'),
    commitment: commitment.toString('hex')
  };
  
  return { note, commitment };
}

// Generate a proof (simplified for demonstration)
function generateProof(notePath, root, recipient, relayer, fee, refund) {
  const note = JSON.parse(fs.readFileSync(notePath, 'utf8'));
  
  // In a real implementation, this would generate a zkSNARK proof
  // For demonstration, we'll just create a dummy proof
  const proof = Buffer.alloc(256, 0);
  
  // Compute nullifier hash
  const nullifierHash = crypto.createHash('sha256')
    .update(Buffer.from(note.nullifier, 'hex'))
    .digest();
  
  return { proof, nullifierHash };
}

// Initialize a new Tornado instance
async function initialize(wallet, denomination, merkleTreeHeight) {
  console.log(`Initializing Tornado instance with denomination ${denomination} and height ${merkleTreeHeight}...`);
  
  // Create a new account for the Tornado instance
  const tornadoInstance = Keypair.generate();
  
  // Create the instruction data
  const instructionData = Buffer.from([
    0, // Initialize instruction
    ...new BN(denomination).toArray('le', 8),
    merkleTreeHeight
  ]);
  
  // Create the transaction
  const transaction = new Transaction().add({
    keys: [
      { pubkey: wallet.publicKey, isSigner: true, isWritable: true },
      { pubkey: tornadoInstance.publicKey, isSigner: true, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }
    ],
    programId: PROGRAM_ID,
    data: instructionData
  });
  
  // Send the transaction
  const signature = await sendAndConfirmTransaction(
    connection,
    transaction,
    [wallet, tornadoInstance]
  );
  
  console.log(`Transaction signature: ${signature}`);
  console.log(`Tornado instance created: ${tornadoInstance.publicKey.toBase58()}`);
  
  return tornadoInstance.publicKey;
}

// Deposit funds into a Tornado instance
async function deposit(wallet, tornadoInstancePubkey, commitment, amount) {
  console.log(`Depositing ${amount} SOL to Tornado instance ${tornadoInstancePubkey}...`);
  
  // Convert commitment to Buffer
  const commitmentBuffer = Buffer.from(commitment, 'hex');
  
  // Create the instruction data
  const instructionData = Buffer.from([
    1, // Deposit instruction
    ...commitmentBuffer
  ]);
  
  // Get the Merkle tree account
  const merkleTreeSeed = Buffer.from([
    ...Buffer.from('merkle_tree', 'utf8'),
    ...new PublicKey(tornadoInstancePubkey).toBuffer(),
    0
  ]);
  const [merkleTreePubkey] = await PublicKey.findProgramAddress(
    [merkleTreeSeed],
    PROGRAM_ID
  );
  
  // Create the transaction
  const transaction = new Transaction().add({
    keys: [
      { pubkey: wallet.publicKey, isSigner: true, isWritable: true },
      { pubkey: new PublicKey(tornadoInstancePubkey), isSigner: false, isWritable: true },
      { pubkey: merkleTreePubkey, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }
    ],
    programId: PROGRAM_ID,
    data: instructionData
  });
  
  // Add a transfer instruction to send SOL
  transaction.add(
    SystemProgram.transfer({
      fromPubkey: wallet.publicKey,
      toPubkey: new PublicKey(tornadoInstancePubkey),
      lamports: amount * 1_000_000_000
    })
  );
  
  // Send the transaction
  const signature = await sendAndConfirmTransaction(
    connection,
    transaction,
    [wallet]
  );
  
  console.log(`Transaction signature: ${signature}`);
  console.log(`Deposit successful!`);
}

// Withdraw funds from a Tornado instance
async function withdraw(wallet, tornadoInstancePubkey, proof, root, nullifierHash, recipient, relayer, fee, refund) {
  console.log(`Withdrawing from Tornado instance ${tornadoInstancePubkey}...`);
  
  // Convert parameters to Buffers
  const proofBuffer = Buffer.from(proof);
  const rootBuffer = Buffer.from(root, 'hex');
  const nullifierHashBuffer = Buffer.from(nullifierHash, 'hex');
  const recipientPubkey = new PublicKey(recipient);
  const relayerPubkey = new PublicKey(relayer);
  
  // Create the instruction data
  const instructionData = Buffer.from([
    2, // Withdraw instruction
    ...new BN(proofBuffer.length).toArray('le', 4),
    ...proofBuffer,
    ...rootBuffer,
    ...nullifierHashBuffer,
    ...recipientPubkey.toBuffer(),
    ...relayerPubkey.toBuffer(),
    ...new BN(fee).toArray('le', 8),
    ...new BN(refund).toArray('le', 8)
  ]);
  
  // Get the Merkle tree account
  const merkleTreeSeed = Buffer.from([
    ...Buffer.from('merkle_tree', 'utf8'),
    ...new PublicKey(tornadoInstancePubkey).toBuffer(),
    0
  ]);
  const [merkleTreePubkey] = await PublicKey.findProgramAddress(
    [merkleTreeSeed],
    PROGRAM_ID
  );
  
  // Create the transaction
  const transaction = new Transaction().add({
    keys: [
      { pubkey: wallet.publicKey, isSigner: true, isWritable: true },
      { pubkey: new PublicKey(tornadoInstancePubkey), isSigner: false, isWritable: true },
      { pubkey: merkleTreePubkey, isSigner: false, isWritable: true },
      { pubkey: recipientPubkey, isSigner: false, isWritable: true },
      { pubkey: relayerPubkey, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }
    ],
    programId: PROGRAM_ID,
    data: instructionData
  });
  
  // Send the transaction
  const signature = await sendAndConfirmTransaction(
    connection,
    transaction,
    [wallet]
  );
  
  console.log(`Transaction signature: ${signature}`);
  console.log(`Withdrawal successful!`);
}

// Set up the CLI
program
  .version('0.1.0')
  .description('Tornado Cash CLI for Solana');

program
  .command('generate-commitment')
  .description('Generate a new commitment')
  .action(() => {
    const { note, commitment } = generateCommitment();
    const notePath = `./note-${Date.now()}.json`;
    fs.writeFileSync(notePath, JSON.stringify(note, null, 2));
    
    console.log(`Note saved to ${notePath}`);
    console.log(`Commitment: ${commitment.toString('hex')}`);
  });

program
  .command('initialize')
  .description('Initialize a new Tornado instance')
  .option('-k, --keypair <path>', 'Path to keypair file', '~/.config/solana/id.json')
  .option('-d, --denomination <amount>', 'Denomination in SOL', DEFAULT_DENOMINATION)
  .option('-h, --height <height>', 'Merkle tree height', DEFAULT_MERKLE_TREE_HEIGHT)
  .action(async (options) => {
    const wallet = loadWallet(options.keypair);
    await initialize(wallet, options.denomination, options.height);
  });

program
  .command('deposit')
  .description('Deposit funds into a Tornado instance')
  .option('-k, --keypair <path>', 'Path to keypair file', '~/.config/solana/id.json')
  .requiredOption('-i, --instance <pubkey>', 'Tornado instance public key')
  .requiredOption('-c, --commitment <hex>', 'Commitment hex')
  .option('-a, --amount <sol>', 'Amount in SOL', 1)
  .action(async (options) => {
    const wallet = loadWallet(options.keypair);
    await deposit(wallet, options.instance, options.commitment, options.amount);
  });

program
  .command('generate-proof')
  .description('Generate a proof for withdrawal')
  .requiredOption('-n, --note <path>', 'Path to note file')
  .requiredOption('-r, --root <hex>', 'Merkle root hex')
  .requiredOption('-t, --recipient <pubkey>', 'Recipient public key')
  .option('-l, --relayer <pubkey>', 'Relayer public key')
  .option('-f, --fee <amount>', 'Fee in lamports', 0)
  .option('-u, --refund <amount>', 'Refund in lamports', 0)
  .action((options) => {
    const { proof, nullifierHash } = generateProof(
      options.note,
      options.root,
      options.recipient,
      options.relayer || options.recipient,
      options.fee,
      options.refund
    );
    
    console.log(`Proof: ${proof.toString('hex')}`);
    console.log(`Nullifier hash: ${nullifierHash.toString('hex')}`);
  });

program
  .command('withdraw')
  .description('Withdraw funds from a Tornado instance')
  .option('-k, --keypair <path>', 'Path to keypair file', '~/.config/solana/id.json')
  .requiredOption('-i, --instance <pubkey>', 'Tornado instance public key')
  .requiredOption('-p, --proof <hex>', 'Proof hex')
  .requiredOption('-r, --root <hex>', 'Merkle root hex')
  .requiredOption('-n, --nullifier-hash <hex>', 'Nullifier hash hex')
  .requiredOption('-t, --recipient <pubkey>', 'Recipient public key')
  .option('-l, --relayer <pubkey>', 'Relayer public key')
  .option('-f, --fee <amount>', 'Fee in lamports', 0)
  .option('-u, --refund <amount>', 'Refund in lamports', 0)
  .action(async (options) => {
    const wallet = loadWallet(options.keypair);
    await withdraw(
      wallet,
      options.instance,
      Buffer.from(options.proof, 'hex'),
      options.root,
      options.nullifierHash,
      options.recipient,
      options.relayer || options.recipient,
      options.fee,
      options.refund
    );
  });

program.parse(process.argv);