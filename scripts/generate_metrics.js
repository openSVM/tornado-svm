#!/usr/bin/env node

/**
 * Utility for generating transaction metrics for the Tornado-SVM solution
 * This is used by the GitHub Actions workflow to generate detailed metrics
 * for transaction performance and gas usage.
 */

const { Connection, PublicKey } = require('@solana/web3.js');

const args = process.argv.slice(2);
if (args.length < 2) {
  console.error('Usage: node generate_metrics.js <transaction_signature> <rpc_url>');
  process.exit(1);
}

const signature = args[0];
const rpcUrl = args[1];

async function generateMetrics() {
  try {
    const connection = new Connection(rpcUrl, 'confirmed');
    
    // Get transaction details
    console.log(`Fetching metrics for transaction: ${signature}`);
    const tx = await connection.getTransaction(signature, {
      commitment: 'confirmed',
      maxSupportedTransactionVersion: 0
    });

    if (!tx) {
      console.error('Transaction not found');
      process.exit(1);
    }

    // Generate metrics JSON
    const metrics = {
      signature: signature,
      timestamp: new Date(tx.blockTime * 1000).toISOString(),
      slot: tx.slot,
      computeUnitsConsumed: tx.meta.computeUnitsConsumed,
      fee: tx.meta.fee / 1_000_000_000, // Convert lamports to SOL
      status: tx.meta.err ? 'Failed' : 'Success',
      confirmations: tx.confirmations,
      blockTime: tx.blockTime,
      recentBlockhash: tx.transaction.message.recentBlockhash,
      accountKeys: tx.transaction.message.accountKeys.length,
      logMessages: tx.meta.logMessages,
    };

    // Output metrics in JSON format
    console.log(JSON.stringify(metrics, null, 2));
  } catch (error) {
    console.error('Error generating metrics:', error);
    process.exit(1);
  }
}

generateMetrics();