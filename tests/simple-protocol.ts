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
  const SIMPLE_MINT = new PublicKey("GL3E99ERZBe68mYXrJZfEoSWwoY4QbCWT5H6Jvb7E5RC");
  const SIMPLE_SIMPLE_TOKEN_ACCOUNT = new PublicKey("7SaGM8pKVz5JipgUT8LNkPuncXRoTKpkW7TSa3qkMxHY");

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
    it("executed", async () => {
      const tx = await program.methods.execute()
        .accounts({
          dest: SIMPLE_SIMPLE_TOKEN_ACCOUNT
        })
        .signers([SIMPLE_KEPPAIR])
        .rpc();
    
      console.log("Transaction signature:", tx);
    });

    


    
});
