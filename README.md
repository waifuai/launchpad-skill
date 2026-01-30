# Launchpad Skill (`ls`)

A modular Rust CLI designed for AI agents to manage tokenized companies on Solana. This tool provides a high-fidelity interface for corporate operations, integrating equity issuance, debt markets, and derivative trading with a social narrative layer.

## 🚀 Overview

`launchpad-skill` implements a comprehensive "Company-as-a-Protocol" stack on Solana. It allows agents to not just trade tokens, but to operate the entire financial structure of a for-profit entity.

### High-Fidelity Layers:
- **Equity**: Core SPL token issuance and liquidity management.
- **Debt**: Dynamic bond markets where interest rates are driven by `reasoning-pricer` logic.
- **Derivatives**: Native futures and leverage for sophisticated risk management.
- **Narrative**: A "Company Narrative" feed powered by Solana Transaction Memos.

## 🛠 Installation

```bash
# Clone the repository
git clone https://github.com/your-repo/launchpad-skill
cd launchpad-skill

# Build the project
cargo build --release

# The binary will be available at
./target/release/ls --help
```

## 📖 CLI Reference

The CLI is structured into four main subcommands: `equity`, `debt`, `trade`, and `feed`.

### 1. Equity Management
Manage core company shares as SPL tokens.

```bash
# Issue new equity tokens
ls equity issue --name "Acme Corp" --symbol ACME --supply 1000000

# Swap equity tokens in a pool
ls equity swap --from ACME --to SOL --amount 100

# Check status of equity pools
ls equity status --symbol ACME
```

### 2. Debt Markets
Issue and buy corporate bonds with dynamic yields.

```bash
# Issue corporate bonds
ls debt issue-bond --symbol ACME --principal 50000

# Buy bonds from the market
ls debt buy-bond --symbol ACME --amount 1000

# Check current bond yields
ls debt yield --symbol ACME
```

### 3. Derivatives & Trading
Open leveraged positions on company tokens.

```bash
# Open a long position (3x leverage)
ls trade long --symbol ACME --amount 500 --leverage 3.0

# Open a short position
ls trade short --symbol ACME --amount 200 --leverage 2.0

# Liquidate an underwater position
ls trade liquidate --symbol ACME --user <USER_ADDRESS>
```

### 4. Company Narrative (Feed)
View the "narrative" of a company based on on-chain memos.

```bash
# View transaction feed and company narrative
ls feed <COMPANY_ADDRESS>
```

## 🤖 Agent Integration

All commands support the `--json` flag to provide machine-readable output, making it easy for AI agents to parse results and make informed decisions.

```bash
ls equity status --symbol ACME --json
```

## 🏗 Architecture

The project is structured modularly:
- `src/equity/`: Token issuance and swap logic.
- `src/debt/`: Bond market and interest rate calculations.
- `src/trade/`: Futures and leverage engine.
- `src/feed/`: Memo parsing and narrative generation.
- `src/pricing/`: Integration with risk-adjusted pricing models.

## 📄 License

This project is licensed under the MIT-0 License.
