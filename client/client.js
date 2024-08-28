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
    createAssociatedTokenAccountIdempotentInstruction,
    TOKEN_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID
} = require('@solana/spl-token');

const SIMPLE_PROGRAM_ID = new PublicKey('FcZSeqBmwYukKXNQ2MzqdZs9vTaKwvhP2ABLn2F9VzEw');
const simple_token_mint = new PublicKey('DJZ2QJ9x7S4XLR7fvPouR5nZfRXqw92Y7S2BNueZmmde');
const simple_keypair = Keypair.fromSecretKey(new Uint8Array(JSON.parse(fs.readFileSync('/home/seb/MY/KEYS/simple.json', 'utf8'))));

const user_keypair = Keypair.fromSecretKey(new Uint8Array(JSON.parse(fs.readFileSync('/home/seb/MY/TEST_KEYS/user.json', 'utf8'))));
const seed_user_claim_tracker = Buffer.from('user_claim_tracker_pda');
const [user_claim_tracker_pda_pubkey] = PublicKey.findProgramAddressSync([seed_user_claim_tracker, user_keypair.publicKey.toBuffer()], SIMPLE_PROGRAM_ID);

const user1_keypair = Keypair.fromSecretKey(new Uint8Array(JSON.parse(fs.readFileSync('/home/seb/MY/TEST_KEYS/user1.json', 'utf8'))));
const seed_user1_claim_tracker = Buffer.from('user_claim_tracker_pda');
const [user1_claim_tracker_pda_pubkey] = PublicKey.findProgramAddressSync([seed_user1_claim_tracker, user1_keypair.publicKey.toBuffer()], SIMPLE_PROGRAM_ID);

// PDAs
const seed_percent_tracker = Buffer.from('percent_tracker_pda');
const [percent_tracker_pda_pubkey] = PublicKey.findProgramAddressSync([seed_percent_tracker], SIMPLE_PROGRAM_ID);

const seed_wsol_amount = Buffer.from('wsol_amount_pda');
const [wsol_sol_amount_pda_pubkey] = PublicKey.findProgramAddressSync([seed_wsol_amount], SIMPLE_PROGRAM_ID);

const seed_transfer_signer = Buffer.from('transfer_signer_pda');
const [transfer_signer_pda_pubkey] = PublicKey.findProgramAddressSync([seed_transfer_signer], SIMPLE_PROGRAM_ID);

// Placeholders
const simple_pool_wsol_token_account = new PublicKey('EBp3owAovYaG1P9TNKfnqc4wY8FwU8iqTsA4Mprwr3JG');
const simple_lp_token_mint = new PublicKey('Fep9kTWfPCQ6uADqdnLnnvYZ39jH7h1oQZVHdSnU2Nmb');

const connection = new Connection('https://api.devnet.solana.com', 'confirmed');

(async () => {
    try {

        const user_simple_ass_token_account_pubkey = await getAssociatedTokenAddress(
            simple_token_mint,
            user_keypair.publicKey,
        );
        const user1_simple_ass_token_account_pubkey = await getAssociatedTokenAddress(
            simple_token_mint,
            user1_keypair.publicKey,
        );

        const user_simple_lp_ass_token_account_pubkey = await getAssociatedTokenAddress(
            simple_lp_token_mint,
            user_keypair.publicKey,
        );
        const user1_simple_lp_ass_token_account_pubkey = await getAssociatedTokenAddress(
            simple_lp_token_mint,
            user1_keypair.publicKey,
        );

        const simple_instruction_0 = new TransactionInstruction({
            programId: SIMPLE_PROGRAM_ID,
            keys: [
                { pubkey: simple_keypair.publicKey, isSigner: true, isWritable: true },
                { pubkey: percent_tracker_pda_pubkey, isSigner: false, isWritable: true },
                { pubkey: wsol_sol_amount_pda_pubkey, isSigner: false, isWritable: true },
                { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }
            ],
            data: Buffer.from([0]),
        });
        
        const user_instruction_1 = new TransactionInstruction({
            programId: SIMPLE_PROGRAM_ID,
            keys: [
                { pubkey: user_keypair.publicKey, isSigner: true, isWritable: true },
                { pubkey: user_claim_tracker_pda_pubkey, isSigner: false, isWritable: true },
                { pubkey: user_simple_ass_token_account_pubkey, isSigner: false, isWritable: true },
                { pubkey: simple_token_mint, isSigner: false, isWritable: false },
                { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
                { pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
                { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }
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
