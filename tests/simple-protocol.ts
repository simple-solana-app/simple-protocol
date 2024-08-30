import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SimpleProtocol } from "../target/types/simple_protocol";
import { PublicKey } from "@solana/web3.js";
import * as fs from "fs";




describe("simple-protocol", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SimpleProtocol as Program<SimpleProtocol>;

  const SIMPLE = Uint8Array.from(JSON.parse(fs.readFileSync("/home/seb/MY/KEYS/simple.json", "utf-8")));
  const SIMPLE_KEPPAIR = anchor.web3.Keypair.fromSecretKey(SIMPLE);
  const USER = Uint8Array.from(JSON.parse(fs.readFileSync("/home/seb/MY/TEST_KEYS/user.json", "utf-8")));
  const USER_KEPPAIR = anchor.web3.Keypair.fromSecretKey(USER);

  const [percentTrackerPda, percentTrackerBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("percent_tracker")],
    program.programId
  );

  const [wsolBalancePda, wsolBalanceBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("wsol_balance")],
    program.programId
  );

  const [transferAuthorityPda, transferAuthorityBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("transfer_authority")],
    program.programId
  );

  const [programSimpleTokenPda, programSimpleTokenBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("program_simple_token_account")],
    program.programId
  );

  const [userClaimTrackerPda, userClaimTrackerBump] = PublicKey.findProgramAddressSync(
    [USER_KEPPAIR.publicKey.toBuffer()],  // Convert PublicKey to Buffer
    program.programId
  );


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

    // it("Created user_claim_tracker_pda", async () => {
    //   const tx = await program.methods.initalizeUserClaimTracker()
    //     .accounts({
    //       user: USER_KEPPAIR.publicKey
    //     })
    //     .signers([USER_KEPPAIR])
    //     .rpc();
      
    //   console.log("Transaction signature:", tx);
    // })

    it("executed", async () => {
      
      const tx = await program.methods.execute()
        .accounts({
        })
        .signers([SIMPLE_KEPPAIR])
        .rpc();
    
      console.log("Transaction signature:", tx);
    });

    


    
});
