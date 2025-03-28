# Tornado-SVM GitHub Actions Workflows

## Testnet Transaction Metrics Workflow

This workflow automates the process of running Tornado-SVM privacy solution transactions on Solana testnet and generating comprehensive metrics reports.

### Workflow: `tornado_testnet_transaction.yml`

**Purpose:** Execute the complete Tornado-SVM transaction flow on Solana testnet and collect detailed performance metrics.

### Trigger Methods:

1. **Manual Trigger:** Run the workflow on-demand via GitHub UI with configurable parameters
2. **Scheduled Runs:** Automatically runs weekly on Sundays at midnight UTC
3. **Pull Request Trigger:** Runs on PRs to the master branch that modify core files

### Configurable Parameters:

- **Denomination:** Amount of SOL to use in the transaction (default: 1 SOL)
- **Merkle Tree Height:** Height of the Merkle tree for the Tornado instance (default: 20)
- **RPC URL:** Custom Solana RPC URL (defaults to testnet)

### What the Workflow Does:

1. Sets up the Solana toolchain and required dependencies
2. Creates a new Solana wallet and requests an airdrop
3. Deploys the Tornado-SVM program to the Solana testnet
4. Initializes a new Tornado instance
5. Performs a complete deposit and withdrawal flow with zkSNARK proofs
6. Captures detailed metrics at each step including:
   - Execution times for each phase
   - Transaction signatures
   - Compute unit consumption
   - Gas fees
   - Transaction details
7. Generates a comprehensive markdown report with visualizations
8. Creates a GitHub job summary
9. Uploads all reports and raw metrics as artifacts

### Artifacts Generated:

- **transaction_report.md:** Complete markdown report with all metrics and visualizations
- **metrics/*.json:** Raw JSON data for transaction details
- **metrics/execution_times.txt:** Detailed timing measurements for each phase

### Using the Report:

1. Download the artifact from the completed workflow run
2. Open the markdown report to view all metrics and visualizations
3. The report includes:
   - Executive summary
   - Configuration details
   - Transaction logs
   - Detailed metrics for each transaction
   - Explorer links for all on-chain activity
   - Visualizations of the transaction flow and zkSNARK process
   - Solana network stats during the test

### Example Usage

To manually trigger the workflow with custom parameters:

1. Go to the "Actions" tab in the GitHub repository
2. Select "Tornado SVM Testnet Transaction Test" workflow
3. Click "Run workflow"
4. Enter your desired parameters (denomination, Merkle tree height, RPC URL)
5. Click "Run workflow"
6. Once completed, download the artifacts from the workflow run