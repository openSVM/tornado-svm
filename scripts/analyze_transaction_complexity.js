#!/usr/bin/env node

/**
 * Utility for analyzing computational complexity of Tornado-SVM transactions
 * Provides a detailed breakdown of the zkSNARK verification costs and other operations
 */

const { Connection, PublicKey } = require('@solana/web3.js');

if (process.argv.length < 3) {
  console.error('Usage: node analyze_transaction_complexity.js <transaction_signature> [rpc_url]');
  process.exit(1);
}

const signature = process.argv[2];
const rpcUrl = process.argv[3] || 'https://api.testnet.solana.com';

async function analyzeTransactionComplexity() {
  try {
    console.log(`Analyzing computational complexity for transaction: ${signature}`);
    const connection = new Connection(rpcUrl, 'confirmed');

    // Get transaction details with parsed information
    const tx = await connection.getParsedTransaction(signature, {
      commitment: 'confirmed',
      maxSupportedTransactionVersion: 0
    });

    if (!tx) {
      console.error('Transaction not found');
      process.exit(1);
    }

    // Extract relevant information for analysis
    const complexity = {
      transactionSignature: signature,
      timestamp: new Date(tx.blockTime * 1000).toISOString(),
      computeUnits: tx.meta.computeUnitsConsumed,
      fee: tx.meta.fee / 1_000_000_000, // lamports to SOL
      instructionCount: tx.transaction.message.instructions.length,
      accountKeys: tx.transaction.message.accountKeys.length,
      operations: []
    };

    // Analyze log messages to identify computationally expensive operations
    if (tx.meta.logMessages) {
      let currentOp = null;
      let instructionIndex = 0;

      for (const log of tx.meta.logMessages) {
        // New instruction begins
        if (log.includes('Program log: Instruction:')) {
          // Save previous operation if exists
          if (currentOp) {
            complexity.operations.push(currentOp);
          }

          // Extract instruction type
          const instructionType = log.includes('Deposit') ? 'Deposit' : 
                                 log.includes('Withdraw') ? 'Withdraw' : 
                                 log.includes('Initialize') ? 'Initialize' : 
                                 'Unknown';
          
          currentOp = {
            index: instructionIndex++,
            type: instructionType,
            program: tx.transaction.message.instructions[instructionIndex-1] && tx.transaction.message.instructions[instructionIndex-1].programId ? tx.transaction.message.instructions[instructionIndex-1].programId.toString() : 'Unknown',
            subOperations: [],
            computeEstimate: 0
          };
        }
        
        // Extract zkSNARK verification information
        else if (log.includes('Program log: Verifying proof')) {
          if (currentOp) {
            currentOp.subOperations.push({
              name: 'zkSNARK Verification',
              estimated_compute: 'High (50,000+ CUs)',
              details: 'Zero-knowledge proof verification'
            });
            currentOp.computeEstimate += 50000; // Estimate
          }
        }
        
        // Extract Merkle tree operations
        else if (log.includes('Program log: Updating Merkle tree')) {
          if (currentOp) {
            currentOp.subOperations.push({
              name: 'Merkle Tree Update',
              estimated_compute: 'Medium (10,000-20,000 CUs)',
              details: 'Insert commitment and recalculate path'
            });
            currentOp.computeEstimate += 15000; // Estimate
          }
        }
        
        // Extract hash calculations
        else if (log.includes('Program log: Computing hash')) {
          if (currentOp) {
            currentOp.subOperations.push({
              name: 'MiMC Hash Calculation',
              estimated_compute: 'Medium (5,000-10,000 CUs)',
              details: 'Cryptographic hash using MiMC'
            });
            currentOp.computeEstimate += 7500; // Estimate
          }
        }
      }

      // Add the last operation if it exists
      if (currentOp) {
        complexity.operations.push(currentOp);
      }
    }

    // Calculate estimated proportion of compute units for each operation
    if (complexity.operations.length > 0 && complexity.computeUnits) {
      let totalEstimated = complexity.operations.reduce(
        (sum, op) => sum + op.computeEstimate, 0
      );

      // If our estimates are way off, adjust them proportionally
      if (totalEstimated > 0) {
        const scaleFactor = complexity.computeUnits / totalEstimated;
        complexity.operations.forEach(op => {
          op.computeEstimate = Math.round(op.computeEstimate * scaleFactor);
          op.percentage = Math.round((op.computeEstimate / complexity.computeUnits) * 100);
        });
      }
    }

    // Output complexity analysis as JSON
    console.log(JSON.stringify(complexity, null, 2));
  } catch (error) {
    console.error('Error analyzing transaction complexity:', error);
    process.exit(1);
  }
}

analyzeTransactionComplexity();
