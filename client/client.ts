import { Connection, Keypair, PublicKey, Transaction, TransactionInstruction, TransactionSignature, clusterApiUrl, sendAndConfirmTransaction } from '@solana/web3.js';
import { error } from 'console';
import fs from 'fs';


let programId = new PublicKey("24x6XDgxxZgSzuAefWmx7WAppzBfgCSHtxAkDtpALbq1");

const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');

const secretKey = JSON.parse(fs.readFileSync('../../KEYS/simple.json', 'utf8'));

// Create a Keypair from the secret key
const payer = Keypair.fromSecretKey(new Uint8Array(secretKey));

main()
    .then(() => {
        console.log("the numbers mason")
    })
    .catch((error) => {
        console.log(error)
    })

async function main() {
    let payer = Keypair.fromSecretKey(new Uint8Array(secretKey));

    const tx = await runProgram(payer);

    console.log(tx);
}

async function runProgram(payer:Keypair): Promise<TransactionSignature> {
    const tx = new Transaction();

    const instruction = new TransactionInstruction({
        keys: [],
        programId,
    });

    tx.add(instruction);

    const txSig = await sendAndConfirmTransaction(
        connection,
        tx,
        [payer]
    )

    return txSig;
}
