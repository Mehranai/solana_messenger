import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { assert } from "chai";

describe("encrypted_messenger", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.EncryptedMessenger as Program;
  const participantA = provider.wallet.publicKey;
  const participantB = web3.Keypair.generate().publicKey;

  function orderedPair(left: web3.PublicKey, right: web3.PublicKey) {
    return Buffer.compare(left.toBuffer(), right.toBuffer()) < 0
      ? [left, right]
      : [right, left];
  }

  function conversationPda(first: web3.PublicKey, second: web3.PublicKey) {
    return web3.PublicKey.findProgramAddressSync(
      [Buffer.from("conversation"), first.toBuffer(), second.toBuffer()],
      program.programId
    );
  }

  function messagePda(conversation: web3.PublicKey, messageId: anchor.BN) {
    const idBytes = messageId.toArrayLike(Buffer, "le", 8);
    return web3.PublicKey.findProgramAddressSync(
      [Buffer.from("message"), conversation.toBuffer(), idBytes],
      program.programId
    );
  }

  function messageDataPda(message: web3.PublicKey) {
    return web3.PublicKey.findProgramAddressSync(
      [Buffer.from("message_data"), message.toBuffer()],
      program.programId
    );
  }

  it("creates a conversation and stores encrypted message bytes", async () => {
    const [first, second] = orderedPair(participantA, participantB);
    const [conversation] = conversationPda(first, second);

    await program.methods
      .initializeConversation(first, second)
      .accounts({ conversation, payer: participantA })
      .rpc();

    const before = await program.account.conversation.fetch(conversation);
    const [message] = messagePda(conversation, before.nextMessageId);
    const [messageData] = messageDataPda(message);

    const ciphertext = Buffer.from("presentation ciphertext bytes");
    const nonce = Array.from(Buffer.alloc(24, 7));

    await program.methods
      .sendMessage(ciphertext, nonce)
      .accounts({
        sender: participantA,
        conversation,
        message,
        messageData,
      })
      .rpc();

    const storedMessage = await program.account.message.fetch(message);
    const storedData = await program.account.messageData.fetch(messageData);
    const after = await program.account.conversation.fetch(conversation);

    assert(storedMessage.sender.equals(participantA));
    assert(storedMessage.dataAccount.equals(messageData));
    assert.deepEqual(Buffer.from(storedData.ciphertext), ciphertext);
    assert.equal(after.nextMessageId.toNumber(), before.nextMessageId.toNumber() + 1);
  });
});
