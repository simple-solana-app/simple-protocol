const fs = require('fs');
const {
    Connection,
    Keypair,
    SystemProgram,
    Transaction,
    PublicKey,
    sendAndConfirmTransaction,
    TransactionInstruction,
    ComputeBudgetProgram
} = require('@solana/web3.js');
const {
    getAssociatedTokenAddress,
    ASSOCIATED_TOKEN_PROGRAM_ID,
    TOKEN_2022_PROGRAM_ID: TOKEN_PROGRAM_ID
} = require('@solana/spl-token');

const SIMPLE_PROGRAM_ID = new PublicKey('3dR1XnxdC7evkcFRLtdbQhK9UfN36m5tWcJMp4nnz3pz');
const simple_token_mint = new PublicKey('9CigozmpiDkUCXBjWojV1hi4jj6Sc47LXfs3aXKjhv2j');
const simple = Keypair.fromSecretKey(new Uint8Array(JSON.parse(fs.readFileSync('/home/seb/MY/KEYS/simple.json', 'utf8'))));

const seed_user_claim_tracker = Buffer.from('user_claim_tracker');

const user_keypair = Keypair.fromSecretKey(new Uint8Array(JSON.parse(fs.readFileSync('/home/seb/MY/TEST_KEYS/user.json', 'utf8'))));
const [user_claim_tracker_pda_pubkey] = PublicKey.findProgramAddressSync([seed_user_claim_tracker, user_keypair.publicKey.toBuffer()], SIMPLE_PROGRAM_ID);

const user1_keypair = Keypair.fromSecretKey(new Uint8Array(JSON.parse(fs.readFileSync('/home/seb/MY/TEST_KEYS/user1.json', 'utf8'))));
const [user1_claim_tracker_pda_pubkey] = PublicKey.findProgramAddressSync([seed_user_claim_tracker, user1_keypair.publicKey.toBuffer()], SIMPLE_PROGRAM_ID);

// PDAs
const seed_percent_tracker = Buffer.from('percent_tracker');
const [percent_tracker_pda_pubkey] = PublicKey.findProgramAddressSync([seed_percent_tracker], SIMPLE_PROGRAM_ID);

const seed_wsol_amount = Buffer.from('wsol_amount');
const [wsol_amount_pda_pubkey] = PublicKey.findProgramAddressSync([seed_wsol_amount], SIMPLE_PROGRAM_ID);

const seed_authority = Buffer.from('authority');
const [authority_pda_pubkey] = PublicKey.findProgramAddressSync([seed_authority], SIMPLE_PROGRAM_ID);

const seed_simple = Buffer.from('simple');
const [program_simple_pda_pubkey] = PublicKey.findProgramAddressSync([seed_simple], SIMPLE_PROGRAM_ID)

const raydium_pool_wsol_token_account = new PublicKey('EBp3owAovYaG1P9TNKfnqc4wY8FwU8iqTsA4Mprwr3JG');
const raydium_lp_token_mint = new PublicKey('Fep9kTWfPCQ6uADqdnLnnvYZ39jH7h1oQZVHdSnU2Nmb');

const connection = new Connection('https://api.devnet.solana.com', 'confirmed');

console.log("percent_tracker_pda:", percent_tracker_pda_pubkey.toBase58());
console.log("wsol_amount_pda:", wsol_amount_pda_pubkey.toBase58());
console.log("authority_pda:", authority_pda_pubkey.toBase58());
console.log("program_simple_pda:", program_simple_pda_pubkey.toBase58());
console.log("user_claim_tracker_pda:", user_claim_tracker_pda_pubkey.toBase58());

(async () => {
    try {
        const user_simple_ata_pubkey = await getAssociatedTokenAddress(
            simple_token_mint,
            user_keypair.publicKey,
        );
        console.log("user_simple_ass_token_account:", user_simple_ata_pubkey.toBase58());

        const user1_simple_ass_token_account_pubkey = await getAssociatedTokenAddress(
            simple_token_mint,
            user1_keypair.publicKey,
        );
        const user_simple_lp_ass_token_account_pubkey = await getAssociatedTokenAddress(
            raydium_lp_token_mint,
            user_keypair.publicKey,
        );
        const user1_simple_lp_ass_token_account_pubkey = await getAssociatedTokenAddress(
            raydium_lp_token_mint,
            user1_keypair.publicKey,
        );

        const simple_instruction_0 = new TransactionInstruction({
            programId: SIMPLE_PROGRAM_ID,
            keys: [
                { pubkey: simple.publicKey, isSigner: true, isWritable: true },
                { pubkey: percent_tracker_pda_pubkey, isSigner: false, isWritable: true },
                { pubkey: wsol_amount_pda_pubkey, isSigner: false, isWritable: true },
                { pubkey: authority_pda_pubkey, isSigner: false, isWritable: true },
                { pubkey: program_simple_pda_pubkey, isSigner: false, isWritable: true },
                { pubkey: simple_token_mint, isSigner: false, isWritable: true },
                { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: true },
                { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }
            ],
            data: Buffer.from([0]),
        });
        
        const user_instruction_1 = new TransactionInstruction({
            programId: SIMPLE_PROGRAM_ID,
            keys: [
                { pubkey: user_keypair.publicKey, isSigner: true, isWritable: true },
                { pubkey: user_claim_tracker_pda_pubkey, isSigner: false, isWritable: true },
                { pubkey: user_simple_ata_pubkey, isSigner: false, isWritable: true },
                { pubkey: simple_token_mint, isSigner: false, isWritable: true },
                { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: true },
                { pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: true },
                { pubkey: SystemProgram.programId, isSigner: false, isWritable: true }
            ],
            data: Buffer.from([1]),
        });

        const compute_budget_ix = ComputeBudgetProgram.setComputeUnitLimit({
            units: 500000,
        });
        
        const transaction_with_instruction = new Transaction()
            .add(compute_budget_ix)
            .add(user_instruction_1);
        
        const signature = await sendAndConfirmTransaction(connection, transaction_with_instruction, [user_keypair]);

        console.log('Program transaction confirmed with signature:', signature);
    } catch (error) {
        console.error('Error:', error);
    }
})();
