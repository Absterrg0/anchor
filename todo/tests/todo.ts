import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import assert from "assert";
import { Todo } from "../target/types/Todo";

describe("Todo List", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Todo as Program<Todo>;

  const user = anchor.web3.Keypair.generate();
  let todoPda: anchor.web3.PublicKey;
  let bump: number;

  before("Airdrop SOL & Init PDA", async () => {
    // Fund user
    const tx = await provider.connection.requestAirdrop(
      user.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(tx);

    // Derive PDA
    [todoPda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("todo"), user.publicKey.toBuffer()],
      program.programId
    );

    // Call initialize
    await program.methods.init()
      .accounts({
        todoAccount: todoPda,
        signer: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();
  });

  it("Adds a task", async () => {
    await program.methods
      .addTodo("learn solana")
      .accounts({
        signer: user.publicKey,
        todoAccount: todoPda,
      })
      .signers([user])
      .rpc();

    const account = await program.account.onChainData.fetch(todoPda);
    assert.equal(account.todos.length, 1);
    assert.equal(account.todos[0].description, "learn solana");
    assert.equal(account.todos[0].isComplete, false);
  });

  it("Toggles a task", async () => {
    await program.methods
      .markTodoAsCompleted(1)
      .accounts({
        signer: user.publicKey,
        todoAccount: todoPda,
      })
      .signers([user])
      .rpc();

    const account = await program.account.onChainData.fetch(todoPda);
    assert.equal(account.todos[0].isComplete, true);
  });

  it("Deletes a task", async () => {
    await program.methods
      .deleteTodo(1)
      .accounts({
        signer: user.publicKey,
        todoAccount: todoPda,
      })
      .signers([user])
      .rpc();

    const account = await program.account.onChainData.fetch(todoPda);
    assert.equal(account.todos.length, 0);
  });
});
