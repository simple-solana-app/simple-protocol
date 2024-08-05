import * as fs from 'fs';
import { Connection, PublicKey, Keypair, Transaction, TransactionInstruction, sendAndConfirmTransaction } from '@solana/web3.js';

// Replace with your program's ID
const PROGRAM_ID = new PublicKey('24x6XDgxxZgSzuAefWmx7WAppzBfgCSHtxAkDtpALbq1');

async function main(): Promise<void> {
    // Connect to the Solana Devnet
    const connection = new Connection('https://api.devnet.solana.com', 'confirmed');

    // Load the payer keypair from file
    const payerKeypairData = JSON.parse(fs.readFileSync('../../../KEYS/simple.json').toString()) as number[];
    const secretKey = Uint8Array.from(payerKeypairData);
    const payer = Keypair.fromSecretKey(secretKey);

    console.log('Payer public key:', payer.publicKey.toBase58());

    // Create the transaction instruction to call the program
    const transaction = new Transaction().add(
        new TransactionInstruction({
            keys: [],
            programId: PROGRAM_ID,
            data: Buffer.from([]), // Empty instruction data as the program doesn't need any
        })
    );

    const { blockhash } = await connection.getLatestBlockhash();
    transaction.recentBlockhash = blockhash;

    // Send the transaction
    transaction.feePayer = payer.publicKey;
    await transaction.sign(payer);
    const signature = await sendAndConfirmTransaction(connection, transaction, [payer]);

    console.log('Transaction signature:', signature);
}

main().catch(err => {
    console.error('Error:', err);
});
