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
const {
    TOKEN_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID,
    getAssociatedTokenAddress,
    createAssociatedTokenAccountIdempotentInstruction
} = require('@solana/spl-token');

const simpleKeypair = Keypair.fromSecretKey(new Uint8Array(JSON.parse(fs.readFileSync('/home/seb/MY/KEYS/simple.json', 'utf8'))));
const userKeypair = Keypair.fromSecretKey(new Uint8Array(JSON.parse(fs.readFileSync('/home/seb/MY/TEST_KEYS/user.json', 'utf8'))));

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

const programSimpleTokenAssAccount = new PublicKey('aQLR781cvYJGrcdhedA1W7XCtBN4HBwwsYYbubCL6wK');

const raydiumPoolWsolTokenAccount = new PublicKey('EBp3owAovYaG1P9TNKfnqc4wY8FwU8iqTsA4Mprwr3JG');
const fluxbeamPoolWsolTokenAccount = new PublicKey('9p3pKXRPz2EZZw1Wyv7uwV1MZDRwmnLT3Pv5qEt7zgdq');

const connection = new Connection('https://api.devnet.solana.com', 'confirmed');

(async () => {
    try {
        // Compute the ATA address for the user
        const userSimpleAssTokenAccountPubkey = await getAssociatedTokenAddress(
            simpleTokenMint,
            userKeypair.publicKey,
        );

        // Fetch the ATA account info
        const associatedTokenAccountInfo = await connection.getAccountInfo(userSimpleAssTokenAccountPubkey);

        if (!associatedTokenAccountInfo) {
            // ATA doesn't exist, create it
            const transaction = new Transaction();

            const createATAInstruction = createAssociatedTokenAccountIdempotentInstruction(
                userKeypair.publicKey,
                userSimpleAssTokenAccountPubkey,
                userKeypair.publicKey,
                simpleTokenMint,
            );

            transaction.add(createATAInstruction);

            // Send and confirm the transaction
            const signature = await sendAndConfirmTransaction(connection, transaction, [userKeypair]);
            console.log('Associated token account created with signature:', signature);
        }

        // Now that the ATA is guaranteed to exist, include it in the program instruction
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
                { pubkey: userSimpleAssTokenAccountPubkey, isSigner: false, isWritable: true },
                { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
                { pubkey: simpleTokenMint, isSigner: false, isWritable: false },
                { pubkey: raydiumPoolWsolTokenAccount, isSigner: false, isWritable: false },
                { pubkey: fluxbeamPoolWsolTokenAccount, isSigner: false, isWritable: false },
            ],
            data: new Uint8Array([]), // Your program-specific data goes here
        });

        const transactionWithInstruction = new Transaction().add(instruction);

        // Send and confirm the transaction with the instruction that includes the ATA
        const signature = await sendAndConfirmTransaction(connection, transactionWithInstruction, [userKeypair]);
        console.log('Program transaction confirmed with signature:', signature);
    } catch (error) {
        console.error('Error:', error);
    }
})();
