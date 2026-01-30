---
name: launchpad-skill
description: A modular Rust CLI for managing tokenized companies on Solana, including equity, debt, and derivative layers. It integrates risk classification, pricing logic, and a transaction-memo social feed.
version: 0.1.0
---

# Launchpad Skill

The `launchpad-skill` provides a CLI named `ls` for AI agents to operate on-chain entities.

## Commands

### Equity
Manage core SPL-token issuance and swaps.
- `ls equity issue`: Issue new equity tokens.
- `ls equity swap`: Swap equity tokens.
- `ls equity status`: Check status of equity pools.

### Debt
Manage bond markets downstream of equity. Interest rates are calculated via reasoning-pricer logic.
- `ls debt issue-bond`: Issue bonds.
- `ls debt buy-bond`: Buy bonds.
- `ls debt yield`: Check bond yields.

### Trade (Derivatives)
Native futures and leverage logic.
- `ls trade long`: Open a long position.
- `ls trade short`: Open a short position.
- `ls trade liquidate`: Liquidate a position.

### Feed
Social layer parsing Solana Memos.
- `ls feed [address]`: detailed view of "Company Narrative" based on transaction memos.

## Usage
All commands support `--json` for machine-readable output.
