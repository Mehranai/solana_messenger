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

Install the Solana CLI and Anchor CLI, then run:

```bash
anchor build
anchor test
```

The checked-in program id is a placeholder. When you generate a real keypair, run `anchor keys sync` so `declare_id!`, `Anchor.toml`, and the generated IDL agree.

On Windows PowerShell, prefer:

```powershell
.\install-tools.ps1
.\build.ps1
```

See [docs/local-development.md](docs/local-development.md) for the Solana `1.18.20` setup notes.

after this we want to test the validation of code running on Typescript:

```powershell
.\build.ps1 -Test  
solana-test-validator --reset --faucet-sol 1000000 --faucet-per-request-sol-cap 100 --faucet-per-time-sol-cap 1000
```
