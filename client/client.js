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

// Load the payer account (simple) from JSON file
const simpleKeypair = Keypair.fromSecretKey(new Uint8Array(JSON.parse(fs.readFileSync('/home/seb/MY/KEYS/simple.json', 'utf8'))));

// Replace these with your actual program ID
const PROGRAM_ID = new PublicKey('24x6XDgxxZgSzuAefWmx7WAppzBfgCSHtxAkDtpALbq1');

// Define the seed for the PDA and derive the public key
const seed = Buffer.from('percent_tracker_pda'); // Seed for PDA
const [percentTrackerPDA] = PublicKey.findProgramAddressSync([seed], PROGRAM_ID);

const SYSTEM_PROGRAM_ID = SystemProgram.programId;

// Create a new connection to the cluster
const connection = new Connection('https://api.devnet.solana.com', 'confirmed');

const hasClaimAccount = false; // or false
const hasSimpleTokenAccount = false; // or false

// For InitRequiredProgramAccounts
const initRequiredProgramAccountsData = new Uint8Array([0,
    hasClaimAccount,
    hasSimpleTokenAccount
]);

const initRequiredUserAccountsAndExecuteData = new Uint8Array([
    1, // variant for InitRequiredUserAccountsAndExecute
    hasClaimAccount, // has_claim_account
    hasSimpleTokenAccount // has_simple_token_account
]);

// For Execute
const executeData = new Uint8Array([2,
    hasClaimAccount,
    hasSimpleTokenAccount
]);

// Choose the appropriate instruction data
const instructionData = initRequiredUserAccountsAndExecuteData; // or initRequiredProgramAccountsData or executeData

// Define the instruction
const instruction = new TransactionInstruction({
    programId: PROGRAM_ID,
    keys: [
        { pubkey: simpleKeypair.publicKey, isSigner: true, isWritable: true }, // Assume payer is the SIMPLE_ACCOUNT
        { pubkey: percentTrackerPDA, isSigner: false, isWritable: true },
        { pubkey: SYSTEM_PROGRAM_ID, isSigner: false, isWritable: false },
    ],
    data: instructionData, // serialized instruction data
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
