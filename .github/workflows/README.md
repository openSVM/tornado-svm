# Tornado-SVM GitHub Actions Workflows

## Build Workflow

### Workflow: `build.yml`

**Purpose:** Build, test, and validate the Tornado-SVM codebase using the Bun JavaScript runtime and Solana build tools.

### Trigger Methods:

1. **On Push:** Runs on all branch pushes and version tags
2. **On Pull Request:** Runs on all pull requests

### What the Workflow Does:

1. Sets up Bun and Rust toolchains
2. Installs Solana build tools
3. Builds the Solana program using Cargo build-sbf
4. Runs program tests
5. Lints the code with Clippy
6. Builds and tests the client

### Technologies Used:

- **Bun:** Fast JavaScript runtime and package manager
- **Rust:** Primary language for the Solana program
- **Solana CLI:** For building and testing Solana programs

### Solana CLI Installation

The workflow automatically installs the Solana CLI using the following process:

```bash
# Install Solana CLI tools
sh -c "$(curl -sSfL https://release.solana.com/v1.16.0/install)"

# Add to GitHub Actions PATH
echo "$HOME/.local/share/solana/install/active_release/bin" >> $GITHUB_PATH

# Also add to current shell session
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
```

This ensures that the Solana binaries are available for all steps in the workflow that require them.

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

1. Sets up Bun runtime and the Solana toolchain
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

### Troubleshooting

#### Solana CLI Not Found

If you encounter the error `solana: command not found`, check the following:

1. Verify that the Solana CLI installation step completed successfully
2. Check that the PATH is correctly set in each step that uses Solana commands
3. The workflow now explicitly adds the Solana binaries to PATH in each step using:
   ```bash
   export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
   ```
4. The transaction script has been enhanced to provide detailed diagnostic information when Solana is not found
5. If problems persist, try using the `SOLANA_PATH` environment variable in the workflow step
