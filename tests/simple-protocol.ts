import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SimpleProtocol } from "../target/types/simple_protocol";
import { PublicKey, ComputeBudgetProgram  } from "@solana/web3.js";
import * as fs from "fs";




describe("simple-protocol", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SimpleProtocol as Program<SimpleProtocol>;

  const SIMPLE_SECKEY = Uint8Array.from(JSON.parse(fs.readFileSync("/home/seb/MY/KEYS/simple.json", "utf-8")));
  const SIMPLE_KEPPAIR = anchor.web3.Keypair.fromSecretKey(SIMPLE_SECKEY);
  
  const USER_SECKEY = Uint8Array.from(JSON.parse(fs.readFileSync("/home/seb/MY/TEST_KEYS/user.json", "utf-8")));
  const USER_KEPPAIR = anchor.web3.Keypair.fromSecretKey(USER_SECKEY);

  const USER_SIMPLE_TOKEN_ACCOUNT = new PublicKey('2RzSzy62Gam6Lx7MckcfT81pE3aRPFvoe9bDF3zugsXF');
  const USER_RAYDIUM_ATA = new PublicKey('FGqHTVefxwtJfvM2S8TtptwpP8pjYW2XaisuUhGyUqmw');
  
  const USER_1_SECKEY = Uint8Array.from(JSON.parse(fs.readFileSync("/home/seb/MY/TEST_KEYS/user_1.json", "utf-8")));
  const USER_1_KEYPAIR = anchor.web3.Keypair.fromSecretKey(USER_1_SECKEY);

  const USER_1_SIMPLE_TOKEN_ACCOUNT = new PublicKey('4pjrgfzU7FntL8Gd1LhAt9MpBZambdi8VdSLo4dAAhgv');
  const USER_1_RAYDIUM_ATA = new PublicKey('WguDYyabSFvEMpQWAJX6MzegzDASfN6ax6NfJwAaUV8');

  // it("Created user_claim_tracker_pda", async () => {
  //   const tx = await program.methods.initalizeUserClaimTracker()
  //     .accounts({
  //       user: USER_1_KEYPAIR.publicKey
  //     })
  //     .signers([USER_1_KEYPAIR])
  //     .rpc();
    
  //   console.log("Transaction signature:", tx);
  // })

  it("executed", async () => {
    const tx = new anchor.web3.Transaction();

    const additionalComputeBudgetInstruction = ComputeBudgetProgram.setComputeUnitLimit({
      units: 1_400_000, 
    });
    tx.add(additionalComputeBudgetInstruction);

    tx.add(await program.methods.execute()
      .accounts({
        user: USER_1_KEYPAIR.publicKey,
        userSimpleTokenAccount: USER_1_SIMPLE_TOKEN_ACCOUNT,
        userRaydiumLpAta: USER_1_RAYDIUM_ATA
      })
      .signers([USER_1_KEYPAIR])
      .instruction());

    const txSignature = await anchor.AnchorProvider.env().sendAndConfirm(tx, [USER_1_KEYPAIR]);
    
    console.log("Transaction signature:", txSignature);
  });
    
});


    // const [percentTrackerPda, percentTrackerBump] = PublicKey.findProgramAddressSync(
    //   [Buffer.from("percent_tracker")],
    //   program.programId
    // );

    // const [wsolBalancePda, wsolBalanceBump] = PublicKey.findProgramAddressSync(
    //   [Buffer.from("wsol_balance")],
    //   program.programId
    // );

    // const [transferAuthorityPda, transferAuthorityBump] = PublicKey.findProgramAddressSync(
    //   [Buffer.from("transfer_authority")],
    //   program.programId
    // );

    // const [programSimpleTokenPda, programSimpleTokenBump] = PublicKey.findProgramAddressSync(
    //   [Buffer.from("program_simple_token_account")],
    //   program.programId
    // );

    // const [userClaimTrackerPda, userClaimTrackerBump] = PublicKey.findProgramAddressSync(
    //   [USER_KEPPAIR.publicKey.toBuffer()],  // Convert PublicKey to Buffer
    //   program.programId
    // );

    // it("Most Program PDAs initialized", async () => {
    //   const tx = await program.methods.initializeMostProgramAccounts()
    //     .accounts({
    //       simple: SIMPLE_KEPPAIR.publicKey,
    //       percentTracker: percentTrackerPda,
    //       wsolBalance: wsolBalancePda,
    //       systemProgram: anchor.web3.SystemProgram.programId,
    //     })
    //     .signers([SIMPLE_KEPPAIR]) // Add the loaded keypair as the signer
    //     .rpc();
    
    //   console.log("Transaction signature:", tx);
    // });
    // it("Remaining Program PDAs initialized", async () => {
    //   const tx = await program.methods.initializeRemainingProgramAccounts()
    //     .accounts({
    //       simple: SIMPLE_KEPPAIR.publicKey,
    //       transferAuthority: transferAuthorityPda,
    //       simpleMint: SIMPLE_MINT,
    //       programSimpleToken: programSimpleTokenPda,
    //       tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID, 
    //       systemProgram: anchor.web3.SystemProgram.programId,
    //     })
    //     .signers([SIMPLE_KEPPAIR]) // Add the loaded keypair as the signer
    //     .rpc();
    
    //   console.log("Transaction signature:", tx);
    // });


