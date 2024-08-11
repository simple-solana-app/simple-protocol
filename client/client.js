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
const userKeypair = Keypair.fromSecretKey(new Uint8Array(JSON.parse(fs.readFileSync('/home/seb/MY/TEST_KEYS/user.json', 'utf8'))));

// Replace these with your actual program ID
const PROGRAM_ID = new PublicKey('24x6XDgxxZgSzuAefWmx7WAppzBfgCSHtxAkDtpALbq1');

const simpleTokenMint = new PublicKey('DJZ2QJ9x7S4XLR7fvPouR5nZfRXqw92Y7S2BNueZmmde');

// Define the seed for the PDA and derive the public key
const seedPercentTracker = Buffer.from('percent_tracker_pda'); // Seed for PDA
const [percentTrackerPDA] = PublicKey.findProgramAddressSync([seedPercentTracker], PROGRAM_ID);

// Define the seed for the PDA and derive the public key
const seedWsolAmount = Buffer.from('wsol_amount_pda'); // Seed for PDA
const [wsolSolAmountPDA] = PublicKey.findProgramAddressSync([seedWsolAmount], PROGRAM_ID);

// Define the seed for the PDA and derive the public key
const seedTransferSigner = Buffer.from('transfer_signer_pda'); // Seed for PDA
const [transferSignerPDA] = PublicKey.findProgramAddressSync([seedTransferSigner], PROGRAM_ID);

const seedUserClaimTracker = Buffer.from('user_claim_tracker_pda'); // Seed for PDA
const [userClaimTrackerPDA] = PublicKey.findProgramAddressSync([seedUserClaimTracker, userKeypair.publicKey.toBuffer()], PROGRAM_ID);

const programSimpleTokenAssAccount = new PublicKey('aQLR781cvYJGrcdhedA1W7XCtBN4HBwwsYYbubCL6wK'); // might need to be G5ScxD5oeGqDwDftSr6HsvydsycRX4phEEXkiPMsofrJ

// Create a new connection to the cluster
const connection = new Connection('https://api.devnet.solana.com', 'confirmed');

// Define the instruction
const instruction = new TransactionInstruction({
    programId: PROGRAM_ID,
    keys: [
        { pubkey: simpleKeypair.publicKey, isSigner: false, isWritable: true },
        { pubkey: userKeypair.publicKey, isSigner: true, isWritable: true },
        { pubkey: percentTrackerPDA, isSigner: false, isWritable: true },
        { pubkey: wsolSolAmountPDA, isSigner: false, isWritable: true },
        { pubkey: transferSignerPDA, isSigner: false, isWritable: false },
        { pubkey: userClaimTrackerPDA, isSigner: false, isWritable: true },
        { pubkey: programSimpleTokenAssAccount, isSigner: false, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
        { pubkey: simpleTokenMint, isSigner: false, isWritable: false },
    ],
    data: new Uint8Array([]),
});

const transaction = new Transaction().add(instruction);

(async () => {
    try {
        // Send and confirm the transaction
        const signature = await sendAndConfirmTransaction(connection, transaction, [userKeypair]);
        console.log('Transaction confirmed with signature:', signature);
    } catch (error) {
        console.error('Transaction failed:', error);
    }
})();
