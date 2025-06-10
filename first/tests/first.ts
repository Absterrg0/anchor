import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import { First } from '../target/types/first'
import assert from 'assert'



describe('first-contract',()=>{

  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.First as Program<First>;

  const newAccount = anchor.web3.Keypair.generate();
  
  it('Is Initialised',async ()=>{

    const tx = await program.methods.initialise().accounts({
      account:newAccount.publicKey,
      signer:anchor.getProvider().wallet.publicKey
    }).signers([newAccount]).rpc()

    console.log("Transaction set successfuly and account is initialised");
  })

  it("Is double",async ()=>{
    const tx = await program.methods.double().accounts({
      account:newAccount.publicKey,
      signer:anchor.getProvider().wallet.publicKey
    }).rpc()


    console.log("Double function called with transaction signature: ",tx);
    const account = await program.account.dataShape.fetch(newAccount.publicKey);

    assert.equal(account.count,2);
  })



  it("Is added", async()=>{
    const tx = await program.methods.add(5).accounts({
      account:newAccount.publicKey,
      signer:anchor.getProvider().wallet.publicKey
    }).rpc();


    console.log("Add function called with transaction signature: ",tx);

    const account  = await program.account.dataShape.fetch(newAccount.publicKey);

    assert.equal(account.count,7)
  })


  it("Is subtracted",async ()=>{
    const tx = await program.methods.sub(5).accounts({
      account:newAccount.publicKey,
      signer:anchor.getProvider().wallet.publicKey
    }).rpc();


    console.log("Subtract function called with transaction signature: ",tx);

    const account = await program.account.dataShape.fetch(newAccount.publicKey);

    assert.equal(account.count,2);
  })
})