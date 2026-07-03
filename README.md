# Encrypted Messenger Backend

This project is the on-chain backend for a Solana messaging application. The program stores encrypted message bytes in Program Derived Address accounts, while encryption and decryption stay off-chain where secret keys can remain private.

## Architecture

- `Conversation` PDA: one account per pair of users, derived from sorted participant public keys.
- `Message` PDA: metadata for one encrypted message, derived from the conversation and message id.
- `MessageData` PDA: the encrypted payload bytes, derived from the message PDA.

The split between `Message` and `MessageData` is intentional. Metadata remains small and easy to index, while the larger encrypted payload lives in a dedicated data account.

## Why Encryption Is Off-Chain

Solana account data is public. A program should not receive plaintext or private keys because those values would be visible in transactions or program logs. The backend service should encrypt plaintext before calling `send_message`, and authorized readers should fetch the `MessageData` account and decrypt off-chain.

## Account Seeds

```text
Conversation = ["conversation", participant_a, participant_b]
Message      = ["message", conversation, message_id]
MessageData  = ["message_data", message]
```

The two participant keys must be sorted before initializing the conversation. That gives both users the same canonical conversation address and prevents duplicate conversations for `A -> B` and `B -> A`.

## Local Development

This project is pinned to the local Solana `1.18.20` and Anchor `0.30.1` toolchain.

Install these prerequisites:

- Node.js / npm
- Rust / Cargo
- Solana CLI `1.18.20`
- Anchor CLI `0.30.1`

### Install Solana CLI

For native Windows PowerShell:

```powershell
New-Item -ItemType Directory -Force C:\solana-install-tmp
Invoke-WebRequest -Uri https://release.solana.com/v1.18.20/solana-install-init-x86_64-pc-windows-msvc.exe -OutFile C:\solana-install-tmp\solana-install-init.exe
C:\solana-install-tmp\solana-install-init.exe v1.18.20
```

Close and reopen PowerShell, then verify:

```powershell
solana --version
```

Expected version:

```text
solana-cli 1.18.20
```

Create a local wallet if you do not already have one:

```powershell
solana-keygen new -o $env:USERPROFILE\.config\solana\id.json
solana config set --url localhost --keypair $env:USERPROFILE\.config\solana\id.json
```

### Install Anchor CLI

This project uses a project-local Anchor install to avoid conflicts with other global Anchor versions:

```powershell
.\install-tools.ps1
```

This installs Anchor CLI `0.30.1` into `.tools\bin\anchor.exe`.

<<<<<<< HEAD
If you prefer a global Anchor install instead:

```powershell
cargo install --force anchor-cli --version 0.30.1
anchor --version
```

Expected version:

```text
anchor-cli 0.30.1
```

### Install Node Dependencies

Install JavaScript dependencies:

```powershell
npm install
```

In terminal 1, start a local validator with generous local faucet limits:

```powershell
solana-test-validator --reset --faucet-sol 1000000 --faucet-per-request-sol-cap 100 --faucet-per-time-sol-cap 1000
```

In terminal 2, build, deploy, and run tests:

```powershell
.\install-tools.ps1
.\build.ps1 -Test
```

The test command builds the Anchor program, deploys it to the local validator, and runs the TypeScript integration tests.

The checked-in program id is a placeholder. When you generate a real keypair, run `anchor keys sync` so `declare_id!`, `Anchor.toml`, and the generated IDL agree.

See [docs/local-development.md](docs/local-development.md) for the Solana `1.18.20` setup notes.
=======
after this we want to test the validation of code running on Typescript:

```powershell
.\build.ps1 -Test  
solana-test-validator --reset --faucet-sol 1000000 --faucet-per-request-sol-cap 100 --faucet-per-time-sol-cap 1000
```
>>>>>>> 4a5c43375d012475d8654c509348f0f406ec9d3d
