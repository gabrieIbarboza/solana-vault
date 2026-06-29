# Solana Vault

```
   _____________________
  /  __________________ \
 /  /                /  \
|  |   tiny vault    |  |
|  |     __ __       |  |
|  |    |  V  |      |  |
|  |    |_____|      |  |
|  |   (  o o  )     |  |
|  |    \  ^  /      |  |
|  |     \___/       |  |
|  |__________________|  |
 \_______________________/
```

My first vault on Solana devnet. A small, beginner-friendly Anchor program where I learned by doing: create a vault, send SOL into it, and withdraw SOL back out again.

## What it does

- `initialize` creates a vault state account and remembers who owns the vault.
- `deposit` moves SOL from the authority wallet into the vault.
- `withdraw` sends SOL back from the vault to the authority wallet.

Each user gets their own vault, derived with PDAs from two seeds:

- `state` for the vault state account
- `vault` for the SOL-holding vault account

## How it works

The program lives in Anchor and has three main pieces:

1. `VaultState` stores the authority pubkey and the bump for the state PDA.
2. `initialize` creates the state account and the vault PDA for the signer.
3. `deposit` uses a system-program transfer to move lamports from the signer into the vault.
4. `withdraw` checks that the vault has enough SOL, then signs a transfer from the vault PDA back to the signer.

The important idea is that the vault account itself does not store custom data. It is just a PDA-controlled system account holding lamports. The state account is what tracks ownership and helps Anchor verify the correct authority.

## Run it locally

### 1. Install dependencies

```bash
npm install
```

### 2. Build the program

```bash
anchor build
```

### 3. Run the tests

```bash
anchor test
```

The tests use a local LiteSVM environment, so they exercise the same initialize / deposit / withdraw flow without needing a live validator.

### 4. Run the devnet interaction script

Make sure your wallet is funded on devnet and your Anchor config points to devnet.

```bash
ts-node scripts/interact.ts
```

If you prefer, you can also run it through `npx`:

```bash
ANCHOR_WALLET=~/.config/solana/id.json npx ts-node scripts/interact.ts
```

## Devnet setup

This project is configured for devnet in `Anchor.toml`. Before running the script on-chain, check that:

- your wallet file exists at `~/.config/solana/id.json`
- the wallet has devnet SOL
- the program has been built and deployed for the current workspace

Useful commands:

```bash
solana config get
solana airdrop 2
anchor deploy
```

## Project structure

- `programs/vault/src/lib.rs` is the program entrypoint.
- `programs/vault/src/instructions/` contains `initialize`, `deposit`, and `withdraw`.
- `programs/vault/src/state.rs` stores the vault state account.
- `programs/vault/tests/` contains the LiteSVM tests.
- `scripts/interact.ts` runs the devnet demo from TypeScript.

## Why I built it

This was a learning-by-doing project, so the goal was not just to make a vault work, but to understand how Solana programs connect PDAs, system transfers, account constraints, and authority checks into one flow. Small app, real concepts, and enough moving parts to make the learning stick.

## Notes

- Withdrawals are restricted to the vault authority.
- Deposits and withdrawals are both lamport transfers.
- The tests cover happy paths and failure cases, including wrong authority and insufficient funds.
