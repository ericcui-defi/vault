# Solana Vault Program

A Solana program built with [Anchor](https://www.anchor-lang.com/) that allows users to deposit and withdraw SOL through a PDA (Program Derived Address)-controlled vault.

**Program ID:** `6bC76eicZeLaipDf9X7wMmxu8pNvmPbaHtnS7HM3kd6X`

**Deployed on:** Devnet

## How It Works

Each user gets their own vault, derived as a PDA from the seed `"vault"` + the user's public key. The vault stores the owner's authority, a bump seed, and a running total of deposited SOL. Only the original owner can withdraw funds.

### Instructions

| Instruction  | Description |
|-------------|-------------|
| `initialize` | Creates a new vault PDA for the calling wallet |
| `deposit`    | Transfers SOL from the user into their vault |
| `withdraw`   | Transfers SOL from the vault back to the owner |

## Project Structure

```
programs/vault/src/
  lib.rs                    # Program entry point
  state.rs                  # VaultState account definition
  instructions/
    initialize.rs           # Creates the vault PDA
    deposit.rs              # Deposits SOL via System Program CPI
    withdraw.rs             # Withdraws SOL via direct lamport manipulation
app/
  client.ts                 # TypeScript CLI client for interacting with the program
```

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solanalabs.com/cli/install)
- [Anchor CLI](https://www.anchor-lang.com/docs/installation)
- [Node.js](https://nodejs.org/) and Yarn

## Setup

```bash
# Install dependencies
yarn install

# Build the program
anchor build

# Run tests
cargo test
```

## Deploy

```bash
# Configure for devnet
solana config set --url https://api.devnet.solana.com

# Airdrop SOL for deployment (if needed)
solana airdrop 2

# Deploy
anchor deploy
```

## Usage (CLI Client)

Interact with the deployed program using the TypeScript client:

```bash
# Initialize your vault
npx ts-node app/client.ts initialize

# Deposit 0.5 SOL
npx ts-node app/client.ts deposit 0.5

# Check vault balance
npx ts-node app/client.ts balance

# Withdraw 0.5 SOL
npx ts-node app/client.ts withdraw 0.5
```

The client uses the wallet configured at `~/.config/solana/id.json`.

## Tests

Tests use [LiteSVM](https://github.com/LiteSVM/litesvm) to simulate a local Solana environment:

```bash
cargo test
```

- `test_initialize` — verifies vault PDA creation
- `test_deposit` — verifies SOL transfer into the vault with balance assertions
- `test_withdraw` — verifies full deposit and withdrawal cycle
