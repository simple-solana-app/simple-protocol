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
    createAssociatedTokenAccountIdempotentInstruction
} = require('@solana/spl-token');


const simpleKeypair = Keypair.fromSecretKey(new Uint8Array(JSON.parse(fs.readFileSync('/home/seb/MY/KEYS/simple.json', 'utf8'))));
const userKeypair = Keypair.fromSecretKey(new Uint8Array(JSON.parse(fs.readFileSync('/home/seb/MY/TEST_KEYS/user.json', 'utf8'))));
const user1Keypair = Keypair.fromSecretKey(new Uint8Array(JSON.parse(fs.readFileSync('/home/seb/MY/TEST_KEYS/user1.json', 'utf8'))));

const PROGRAM_ID = new PublicKey('24x6XDgxxZgSzuAefWmx7WAppzBfgCSHtxAkDtpALbq1');
const simpleTokenMint = new PublicKey('DJZ2QJ9x7S4XLR7fvPouR5nZfRXqw92Y7S2BNueZmmde');

const seedPercentTracker = Buffer.from('percent_tracker_pda');
const [percentTrackerPDA] = PublicKey.findProgramAddressSync([seedPercentTracker], PROGRAM_ID);

const seedWsolAmount = Buffer.from('wsol_amount_pda');
const [wsolSolAmountPDA] = PublicKey.findProgramAddressSync([seedWsolAmount], PROGRAM_ID);

const seedTransferSigner = Buffer.from('transfer_signer_pda');
const [transferSignerPDA] = PublicKey.findProgramAddressSync([seedTransferSigner], PROGRAM_ID);

const seedUserClaimTracker = Buffer.from('user_claim_tracker_pda');
const [userClaimTrackerPDA] = PublicKey.findProgramAddressSync([seedUserClaimTracker, userKeypair.publicKey.toBuffer()], PROGRAM_ID);

const seedUser1ClaimTracker = Buffer.from('user_claim_tracker_pda');
const [user1ClaimTrackerPDA] = PublicKey.findProgramAddressSync([seedUser1ClaimTracker, user1Keypair.publicKey.toBuffer()], PROGRAM_ID);

const programSimpleTokenAssAccount = new PublicKey('aQLR781cvYJGrcdhedA1W7XCtBN4HBwwsYYbubCL6wK');

const raydiumPoolWsolTokenAccount = new PublicKey('EBp3owAovYaG1P9TNKfnqc4wY8FwU8iqTsA4Mprwr3JG');
const raydiumLpTokenMint = new PublicKey('Fep9kTWfPCQ6uADqdnLnnvYZ39jH7h1oQZVHdSnU2Nmb');

const connection = new Connection('https://api.devnet.solana.com', 'confirmed');

(async () => {
    try {
        // Compute the ATA address for the user
        const user1SimpleAssTokenAccountPubkey = await getAssociatedTokenAddress(
            simpleTokenMint,
            user1Keypair.publicKey,
        );

        //can't claim anyway if they don't hold any
        const user1RaydiumLpAssTokenAccountPubkey = await getAssociatedTokenAddress(
            raydiumLpTokenMint,
            user1Keypair.publicKey,
        );

        // Fetch the ATA account info
        const associatedTokenAccountInfo = await connection.getAccountInfo(user1SimpleAssTokenAccountPubkey);

        if (!associatedTokenAccountInfo) {
            // ATA doesn't exist, create it
            const transaction = new Transaction();

            const createATAInstruction = createAssociatedTokenAccountIdempotentInstruction(
                user1Keypair.publicKey,
                user1SimpleAssTokenAccountPubkey,
                user1Keypair.publicKey,
                simpleTokenMint,
            );

            transaction.add(createATAInstruction);

            // Send and confirm the transaction
            const signature = await sendAndConfirmTransaction(connection, transaction, [user1Keypair]);
            console.log('Associated token account created with signature:', signature);
        }

        // Now that the ATA is guaranteed to exist, include it in the program instruction
        const instruction = new TransactionInstruction({
            programId: PROGRAM_ID,
            keys: [
                { pubkey: simpleKeypair.publicKey, isSigner: false, isWritable: true },
                { pubkey: user1Keypair.publicKey, isSigner: true, isWritable: true },
                { pubkey: percentTrackerPDA, isSigner: false, isWritable: true },
                { pubkey: wsolSolAmountPDA, isSigner: false, isWritable: true },
                { pubkey: transferSignerPDA, isSigner: false, isWritable: false },
                { pubkey: user1ClaimTrackerPDA, isSigner: false, isWritable: true },
                { pubkey: programSimpleTokenAssAccount, isSigner: false, isWritable: true },
                { pubkey: user1SimpleAssTokenAccountPubkey, isSigner: false, isWritable: true },
                { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
                { pubkey: simpleTokenMint, isSigner: false, isWritable: false },
                { pubkey: raydiumPoolWsolTokenAccount, isSigner: false, isWritable: false },
                { pubkey: raydiumLpTokenMint, isSigner: false, isWritable: false },
                { pubkey: user1RaydiumLpAssTokenAccountPubkey, isSigner: false, isWritable: false },
            ],
            data: new Uint8Array([]), // Your program-specific data goes here
        });

        const computeBudgetIx = ComputeBudgetProgram.setComputeUnitLimit({
            units: 300000, // You can request up to 1,400,000 compute units
        });
        
        const transactionWithInstruction = new Transaction()
            .add(computeBudgetIx)
            .add(instruction);
        
        const signature = await sendAndConfirmTransaction(connection, transactionWithInstruction, [user1Keypair]);

        console.log('Program transaction confirmed with signature:', signature);
    } catch (error) {
        console.error('Error:', error);
    }
})();
