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
2. The workflow now adds Solana binaries to GitHub's persistent PATH variable (`$GITHUB_PATH`), ensuring all subsequent steps can access the commands
3. We also add `$HOME/.cargo/bin` to PATH to pick up cargo-build-sbf and cargo-test-sbf
4. The workflow no longer needs explicit PATH exports in each step
5. The transaction script has robust error handling to provide detailed diagnostic information when Solana is not found
6. You can use the `SOLANA_PATH` environment variable to override the default Solana binary location

#### Cargo Lock File Version Compatibility

If you encounter Cargo lock file version compatibility issues:

1. The workflow now explicitly updates Cargo to the latest stable version
2. We've added a specific step that runs `rustup update stable` and `rustup default stable`
3. Cargo version is explicitly checked and logged for troubleshooting
4. The workflow now intelligently checks if the installed Cargo version is compatible with Cargo.lock version 4:
   ```bash
   CARGO_VERSION=$(cargo --version | grep -oE '[0-9]+\.[0-9]+\.[0-9]+')
   MAJOR=$(echo "$CARGO_VERSION" | cut -d'.' -f1)
   MINOR=$(echo "$CARGO_VERSION" | cut -d'.' -f2)
   if [ "$MAJOR" -lt 1 ] || ([ "$MAJOR" -eq 1 ] && [ "$MINOR" -lt 70 ]); then
     # If Cargo is too old, upgrade it again
     curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable --profile minimal
   fi
   ```
5. The workflow automatically regenerates the Cargo.lock file to ensure it uses a format compatible with the current Cargo version
6. After regeneration, it explicitly verifies the lock file format with `grep -q 'version = 4' Cargo.lock`
7. Any existing Cargo.lock is deleted and freshly regenerated to avoid format conflicts
8. Detailed debugging output is provided if the Cargo.lock generation fails

#### Build Command Not Found

If you encounter errors with `cargo build-sbf` or `cargo build-bpf`:

1. The workflow now checks if commands are available using `help` flags
2. It tries both SBF (newer) and BPF (older) variants
3. If needed, it runs `solana-install update` to get the latest build tools
4. PATH is updated to include all possible locations for Cargo and Solana binaries

#### Notifications

The workflow previously used Telegram for notifications, which has been replaced with:

1. Console-based logging for better workflow compatibility
2. No external dependencies or tokens required
3. Clear notification messages in the workflow logs
