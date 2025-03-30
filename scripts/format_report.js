#!/usr/bin/env node

/**
 * Script to format metrics data collected from Tornado-SVM transaction into a markdown report
 * Used by the GitHub Actions workflow to generate the final report artifact
 */

const fs = require('fs');

if (process.argv.length < 4) {
  console.error('Usage: node format_report.js <metrics_json_file> <output_markdown_file>');
  process.exit(1);
}

const metricsFile = process.argv[2];
const outputFile = process.argv[3];

// Create a timestamp for the report
const timestamp = new Date().toISOString();

function formatReport() {
  try {
    // Read the metrics JSON file
    const metricsData = JSON.parse(fs.readFileSync(metricsFile, 'utf8'));
    
    // Format the report in markdown
    let report = `# Tornado-SVM Transaction Metrics Report

`;
    report += `Generated on: ${timestamp}\n\n`;
    
    // Transaction Summary
    report += `## Transaction Summary\n\n`;
    report += `| Metric | Value |\n`;
    report += `| ------ | ----- |\n`;
    report += `| Transaction Signature | \`${metricsData.signature}\` |\n`;
    report += `| Status | ${metricsData.status} |\n`;
    report += `| Timestamp | ${metricsData.timestamp} |\n`;
    report += `| Block | ${metricsData.slot} |\n`;
    report += `| Confirmations | ${metricsData.confirmations} |\n`;
    report += `| Recent Blockhash | ${metricsData.recentBlockhash} |\n\n`;
    
    // Performance Metrics
    report += `## Performance Metrics\n\n`;
    report += `| Metric | Value |\n`;
    report += `| ------ | ----- |\n`;
    report += `| Compute Units Consumed | ${metricsData.computeUnitsConsumed} |\n`;
    report += `| Fee (SOL) | ${metricsData.fee} |\n`;
    report += `| Account Keys Count | ${metricsData.accountKeys} |\n\n`;
    
    // Log Messages
    report += `## Log Messages\n\n`;
    report += `\`\`\`\n`;
    if (metricsData.logMessages && metricsData.logMessages.length > 0) {
      report += metricsData.logMessages.join('\n');
    } else {
      report += 'No log messages available';
    }
    report += `\n\`\`\`\n\n`;
    
    // Generate Merkle Tree visualization if data available
    if (metricsData.merkleTreeHeight) {
      report += `## Merkle Tree Visualization\n\n`;
      report += `\`\`\`mermaid\ngraph TD\n`;
      report += `    Root[Root] --> Level1A[Level 1 Node A]\n`;
      report += `    Root --> Level1B[Level 1 Node B]\n`;
      report += `    Level1A --> Level2A[Level 2 Node A]\n`;
      report += `    Level1A --> Level2B[Level 2 Node B]\n`;
      report += `    Level1B --> Level2C[Level 2 Node C]\n`;
      report += `    Level1B --> Level2D[Level 2 Node D]\n`;
      report += `\`\`\`\n\n`;
    }
    
    // Explorer links
    report += `## Explorer Links\n\n`;
    report += `- [View Transaction on Solana Explorer](https://explorer.solana.com/tx/${metricsData.signature}?cluster=testnet)\n`;
    report += `- [View Block ${metricsData.slot} on Solana Explorer](https://explorer.solana.com/block/${metricsData.slot}?cluster=testnet)\n\n`;
    
    // Save the report to the output file
    fs.writeFileSync(outputFile, report);
    console.log(`Report saved to ${outputFile}`);
  } catch (error) {
    console.error('Error formatting report:', error);
    process.exit(1);
  }
}

formatReport();
