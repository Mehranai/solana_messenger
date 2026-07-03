# Architecture Notes

## Responsibility Split

The on-chain program is responsible for authorization, PDA derivation, message ordering, and encrypted byte storage. It is not responsible for encryption or decryption because Solana transactions and accounts are public.

The backend service should:

1. Derive or retrieve the conversation key material off-chain.
2. Encrypt plaintext with an authenticated encryption scheme.
3. Submit ciphertext and nonce to the Solana program.
4. Fetch ciphertext from `MessageData` and decrypt it for authorized users.

## PDA Model

`Conversation` is the stable parent account. It stores the two participants, the next message id, timestamps, and the bump seed.

`Message` is the indexable metadata account. It stores sender, recipient, timestamp, nonce, cipher scheme, and the address of its data account.

`MessageData` is the payload account. It stores only the message PDA and ciphertext bytes.

## Current Limits

The first version caps ciphertext at 4096 bytes. For longer messages, add chunked `MessageData` accounts:

```text
MessageChunk = ["message_chunk", message, chunk_index]
```

That keeps rent predictable and avoids creating oversized accounts.

## Future Extensions

- Group conversations with a membership PDA per user.
- Message deletion by closing `MessageData` accounts.
- Server-side indexing using `MessageSent` events.
- Optional message receipts using a `Receipt` PDA derived from message and recipient.
- Compression for high-volume message histories.

