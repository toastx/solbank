import { BN } from "bn.js";
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import { publicKey } from "@project-serum/anchor/dist/cjs/utils";
const assert = require('assert')
const anchor = require('@project-serum/anchor')
const {SystemProgram} = anchor.web3


describe("solbank", () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const client = anchor.web3.Keypair.generate()
  const program = anchor.workspace.Solbank;
  let vaultPDA = publicKey;
  before(async () => {
    [vaultPDA] = await anchor.web3.PublicKey.findProgramAddress([
      utf8.encode('vault'),
      provider.wallet.publicKey.toBuffer(), 
    ],
    program.programId
    );
    console.log(vaultPDA);
  });
  it("Create a account", async () => {
  
    const tx =  await program.methods.create()
    .accounts({
           client : vaultPDA,
           user : provider.wallet.publicKey,
           systemProgram: SystemProgram.programID
    })
    .rpc();

    const account = await program.account.client.fetch(vaultPDA);
    assert.equal(account.owner.toBase58(),provider.wallet.publicKey.toBase58());
    console.log("Your transaction signature", tx);

    console.log(account.timestamp.toString());
    assert.equal(account.amount.toNumber(),10);

    
  });

  it("Deposit money", async() =>{

    const tx = await program.methods.deposit(new anchor.BN(1))
    .accounts({
      client : vaultPDA,
      user : provider.wallet.publicKey,
      systemProgram: SystemProgram.programID
    })
    .rpc();

    const account = await program.account.client.fetch(client.publicKey);
    assert.ok(account.amount.equals(new anchor.BN(1)));

    console.log(`You deposited ${account.amount}`)
    console.log("Your transaction signature", tx);
   

  });
  it("Withdraw money", async() =>{

    const tx = await program.methods.withdraw(new anchor.BN(1))
    .accounts({
      client : vaultPDA,
      user : provider.wallet.publicKey,
      systemProgram: SystemProgram.programID
    })
    .rpc();

    const account = await program.account.client.fetch(client.publicKey);
    assert.ok(account.amount.equals(new anchor.BN(9)));

    console.log(`You deposited ${account.amount}`)
    console.log("Your transaction signature", tx);
  
  });



});
