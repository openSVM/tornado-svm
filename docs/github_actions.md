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

**Solana Environment Setup:**
- The workflow automatically installs the Solana CLI tools version 1.16.0
- Adds the Solana binary path to GitHub's persistent PATH variable (`$GITHUB_PATH`)
- Adds `$HOME/.cargo/bin` to PATH to include Solana build tools
- Uses the latest Cargo toolchain with explicit version updates
- Tries multiple command variants for maximum compatibility (SBF/BPF)
- Provides enhanced error reporting when Solana tools are not found

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

## Troubleshooting

### Common Issues

#### Solana CLI Not Found

If your workflow fails with the error `solana: command not found`, check the following:

1. **Verify installation:** Make sure the Solana CLI installation step completed successfully in the logs

2. **GitHub PATH variables:** The workflow now adds Solana paths to `$GITHUB_PATH` for persistence across all steps:
   ```bash
   echo "$HOME/.local/share/solana/install/active_release/bin" >> $GITHUB_PATH
   echo "$HOME/.cargo/bin" >> $GITHUB_PATH
   ```

3. **Installation log:** Look for output from the Solana installation command and verify it completed successfully:
   ```
   sh -c "$(curl -sSfL https://release.solana.com/v1.16.0/install)"
   ```

4. **Enhanced diagnostics:** The `run_tornado_transaction.sh` script includes robust diagnostics when Solana is not found, including checking common installation locations

5. **SOLANA_PATH override:** You can use the `SOLANA_PATH` environment variable to specify a custom location for Solana binaries

#### Cargo Lock File Version Issues

If your workflow fails with errors about Cargo.lock version compatibility:

1. **Update Cargo:** The workflow now explicitly updates Cargo to the latest stable version:
   ```bash
   rustup update stable
   rustup default stable
   ```

2. **Version verification:** The workflow now verifies the Cargo version before proceeding with builds

3. **Compatibility:** These steps ensure compatibility with Cargo.lock version 4 format

#### Solana Build Command Not Found

If you encounter issues with Solana build commands:

1. **Command availability:** The workflow now checks if commands are available using `help` flags rather than checking for the binaries directly:
   ```bash
   if cargo build-sbf --help &> /dev/null; then
     cargo build-sbf
   elif cargo build-bpf --help &> /dev/null; then
     cargo build-bpf
   fi
   ```

2. **Multiple paths:** The workflow adds multiple PATH directories to find all required binaries

3. **Auto-installation:** If build commands aren't found, the workflow runs `solana-install update` to get the latest tools

#### Notification Issues

The previous implementation used Telegram for notifications, which has been eliminated:

1. **Simplified notifications:** All notifications now use console output only

2. **No dependencies:** No external service dependencies or tokens required

3. **Error-free operation:** Guaranteed to work in all CI environments

#### Transaction Failures

If transactions on testnet fail, common causes include:

1. **Airdrop limits:** Testnet has airdrop limits; check if the airdrop succeeded
2. **Testnet stability:** Testnet can occasionally be unstable; try re-running the workflow
3. **RPC errors:** If using a custom RPC URL, verify it's working correctly
