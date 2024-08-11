const fs = require('fs');
const {
    Connection,
    Keypair,
    SystemProgram,
    Transaction,
    PublicKey,
    sendAndConfirmTransaction,
    TransactionInstruction,
} = require('@solana/web3.js');
const { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } = require('@solana/spl-token');

// Load the payer account (simple) from JSON file
const simpleKeypair = Keypair.fromSecretKey(new Uint8Array(JSON.parse(fs.readFileSync('/home/seb/MY/KEYS/simple.json', 'utf8'))));

// Replace these with your actual program ID
const PROGRAM_ID = new PublicKey('24x6XDgxxZgSzuAefWmx7WAppzBfgCSHtxAkDtpALbq1');

const simpleTokenMint = new PublicKey('DJZ2QJ9x7S4XLR7fvPouR5nZfRXqw92Y7S2BNueZmmde');

// Define the seed for the PDA and derive the public key
const seedPercentTracker = Buffer.from('percent_tracker_pda'); // Seed for PDA
const [percentTrackerPDA] = PublicKey.findProgramAddressSync([seedPercentTracker], PROGRAM_ID);

// Define the seed for the PDA and derive the public key
const seedWsolBalance = Buffer.from('wsol_balance_pda'); // Seed for PDA
const [wsolSolBalancePDA] = PublicKey.findProgramAddressSync([seedWsolBalance], PROGRAM_ID);

// Define the seed for the PDA and derive the public key
const seedTransferSigner = Buffer.from('transfer_signer_pda'); // Seed for PDA
const [transferSignerPDA] = PublicKey.findProgramAddressSync([seedTransferSigner], PROGRAM_ID);

const programSimpleTokenAssAccount = new PublicKey('aQLR781cvYJGrcdhedA1W7XCtBN4HBwwsYYbubCL6wK'); // might need to be G5ScxD5oeGqDwDftSr6HsvydsycRX4phEEXkiPMsofrJ

// Create a new connection to the cluster
const connection = new Connection('https://api.devnet.solana.com', 'confirmed');

const hasClaimAccount = false; // or false
const hasSimpleTokenAccount = false; // or false

// For InitRequiredProgramAccounts
const initRequiredProgramAccountsData = new Uint8Array([0,
    null,
    null
]);

const initRequiredUserAccounts = new Uint8Array([
    1, // variant for InitRequiredUserAccounts
    hasClaimAccount, // has_claim_account
    hasSimpleTokenAccount // has_simple_token_account
]);

// For Execute
const executeProgram = new Uint8Array([2,
    null,
    null
]);

// Choose the appropriate instruction data
const instructionData = executeProgram;

// Define the instruction
const instruction = new TransactionInstruction({
    programId: PROGRAM_ID,
    keys: [
        { pubkey: simpleKeypair.publicKey, isSigner: true, isWritable: true }, // Assume payer is the SIMPLE_ACCOUNT
        { pubkey: percentTrackerPDA, isSigner: false, isWritable: true },
        { pubkey: wsolSolBalancePDA, isSigner: false, isWritable: true },
        { pubkey: transferSignerPDA, isSigner: false, isWritable: true },
        { pubkey: programSimpleTokenAssAccount, isSigner: false, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
        { pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
        { pubkey: simpleTokenMint, isSigner: false, isWritable: false },
    ],
    data: instructionData,
});

const transaction = new Transaction().add(instruction);

(async () => {
    try {
        // Send and confirm the transaction
        const signature = await sendAndConfirmTransaction(connection, transaction, [simpleKeypair]);
        console.log('Transaction confirmed with signature:', signature);
    } catch (error) {
        console.error('Transaction failed:', error);
    }
})();
