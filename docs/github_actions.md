# GitHub Actions Workflows for Tornado-SVM

## Overview

This document outlines the GitHub Actions workflows available for automating testing, deployment, and performance monitoring of the Tornado-SVM privacy solution. These workflows help ensure consistent quality and provide detailed metrics for performance analysis.

## Available Workflows

### Build Workflow

**File:** `.github/workflows/build.yml`

**Purpose:**  
This workflow handles the building, testing, and validation of the Tornado-SVM codebase using the Bun JavaScript runtime and Solana build tools. It provides quick feedback on code quality and functionality.

**Key Features:**
- Builds the Solana program using Cargo build-sbf
- Runs integration tests for the program
- Builds and tests the client code
- Performs code linting with Clippy

**Triggers:**
- Executes on all Git pushes to any branch
- Runs on version tags matching `v[0-9]+.[0-9]+.[0-9]+`
- Executes on all pull requests

**Technologies:**
- Uses Bun for JavaScript runtime and package management
- Uses the latest Rust toolchain for Solana program development
- Uses Solana CLI tools for program building and testing

### Tornado Testnet Transaction Test

**File:** `.github/workflows/tornado_testnet_transaction.yml`

**Purpose:**  
This workflow automates the execution of a complete Tornado-SVM transaction flow on the Solana testnet, including deploying the program, initializing a Tornado instance, depositing funds, generating proofs, and withdrawing funds. It captures comprehensive metrics at each step of the process and generates a detailed report.

**Key Features:**
- Executes the full transaction lifecycle on Solana testnet
- Captures detailed timing metrics for each operation
- Monitors compute unit consumption, gas fees, and transaction sizes
- Analyzes the computational complexity of cryptographic operations
- Generates visualizations of the transaction flow and resource usage
- Produces comprehensive markdown reports with embedded metrics
- Saves transaction IDs for verification on Solana Explorer

**Triggers:**
- Manual execution via GitHub UI with configurable parameters
- Scheduled weekly runs every Sunday at midnight UTC
- Automatic execution on PRs that modify core files

**Configuration Options:**
- `denomination`: Amount of SOL to use for the transaction (default: 1)
- `merkle_tree_height`: Height of the Merkle tree (default: 20)
- `rpc_url`: Custom Solana RPC URL (defaults to testnet)

**Artifacts:**
- `tornado-svm-transaction-report`: A comprehensive markdown report with all metrics and visualizations
- Raw metrics data in JSON format for further analysis

### How to Use the Workflow

#### Manual Execution

1. Navigate to the "Actions" tab in the repository
2. Select "Tornado SVM Testnet Transaction Test" from the list of workflows
3. Click the "Run workflow" button
4. Configure parameters as needed:
   - Set the denomination for the transaction (in SOL)
   - Set the Merkle tree height
   - Optionally provide a custom RPC URL
5. Click "Run workflow" to begin execution

#### Accessing Results

1. Once the workflow completes, navigate to the workflow run
2. Scroll to the "Artifacts" section
3. Download the `tornado-svm-transaction-report` artifact
4. The artifact contains a detailed markdown report and JSON metric files

#### Analyzing the Report

The report contains several key sections:

1. **Executive Summary**: High-level overview with key metrics
2. **Configuration**: Details of the environment and settings used
3. **Transaction Log**: Complete log of the transaction execution
4. **Transaction Metrics**: Detailed timing and performance data
5. **Transaction Details**: Raw transaction data from the Solana network
6. **Complexity Analysis**: Breakdown of computational resources used by different operations
7. **Visualizations**: Diagrams showing the transaction flow and resource allocation
8. **Solana Network Metrics**: Network-level stats during testing
9. **Explorer Links**: Direct links to transaction records on Solana Explorer

### Performance Benchmarks

This workflow can be used to establish performance benchmarks for the Tornado-SVM implementation, tracking metrics like:

- Total execution time for the complete transaction cycle
- Computation costs for zkSNARK proof verification
- Gas costs for deposit and withdrawal operations
- Merkle tree operation efficiency

By running this workflow regularly or after significant changes, the team can monitor performance trends and identify optimizations or regressions.

## Development Notes

The workflow uses several custom scripts for capturing and analyzing metrics, executed with Bun:

- `run_tornado_transaction_metrics.sh`: Modified version of the transaction script that captures timing data
- `generate_metrics.js`: Extracts detailed transaction data from the Solana network
- `analyze_transaction_complexity.js`: Analyzes computational complexity of operations
- `format_report.js`: Formats metrics into a readable report

Developers can modify these scripts to capture additional metrics or change how they're presented in the report.
