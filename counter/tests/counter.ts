import * as anchor from "@coral-xyz/anchor";

import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/Counter";
import assert from "assert";


describe("Counter",()=>{
  let provider = anchor.AnchorProvider.env();

  anchor.setProvider(provider);

  let program = anchor.workspace.Counter as Program<Counter>;
  let newAccount = anchor.web3.Keypair.generate();


  before("init",async ()=>{
    await program.methods.initialise(30).signers([newAccount]).accounts({
      signer:provider.wallet.publicKey,
      dataAccount:newAccount.publicKey,
    }).rpc();
  })

  it("Is initialised",async ()=>{
    const dataAccount = await program.account.dataOnChain.fetch(newAccount.publicKey);
    assert.equal(dataAccount.count,30)
})


  it("Is Incrementing ",async ()=>{
    await program.methods.increment(50).accounts({
      dataAccount:newAccount.publicKey
    }).rpc();

    const dataAccount = await program.account.dataOnChain.fetch(newAccount.publicKey);
    assert.equal(dataAccount.count,80);
  })


  it("Is Decrementing",async ()=>{
    await program.methods.decrement(40).accounts({
      dataAccount:newAccount.publicKey,
    }).rpc();

    const dataAccount = await program.account.dataOnChain.fetch(newAccount.publicKey);

    assert.equal(dataAccount.count,40);
  })

})