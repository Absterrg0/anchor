import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import { Voting } from '../target/types/Voting'
import assert from 'assert';



describe("Voting",()=>{

  let provider = anchor.AnchorProvider.env();

  anchor.setProvider(provider);
  
  let program = anchor.workspace.Voting as Program<Voting>;

  const newAccount = anchor.web3.Keypair.generate();
  let votingPda : anchor.web3.PublicKey;
  let bump;

  before("Airdrop",async ()=>{

    const tx = await provider.connection.requestAirdrop(newAccount.publicKey,10 * anchor.web3.LAMPORTS_PER_SOL);

    await provider.connection.confirmTransaction(tx);
    [votingPda,bump] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from('poll'),provider.wallet.publicKey.toBuffer()],program.programId)


    await program.methods.initialise("Egg came before chicken").accounts({
      signer:provider.wallet.publicKey,
    }).rpc();

    console.log("PDA created at:", votingPda)


  })


  it("upvotes",async ()=>{
    await program.methods.vote({upvote: {}}).accounts({
      signer:provider.wallet.publicKey,
      votingAccount:votingPda
    }).rpc();

    const dataOnChain = await program.account.votingAccount.fetch(votingPda);
    assert.equal(dataOnChain.title,"Egg came before chicken");
    assert.equal(dataOnChain.votes,1)


  })


  it("downvotes",async ()=>{
    await program.methods.vote({downvote: {}}).accounts({
      signer:provider.wallet.publicKey,
      votingAccount:votingPda
    }).rpc();

    const dataOnChain = await program.account.votingAccount.fetch(votingPda);
    assert.equal(dataOnChain.title,"Egg came before chicken");
    assert.equal(dataOnChain.votes,0)


  })
})


