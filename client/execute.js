const fs = require('fs');
const {
    Connection,
    PublicKey,
    Keypair,
    TransactionInstruction,
    Transaction,
    sendAndConfirmTransaction,
    ComputeBudgetProgram
} = require('@solana/web3.js');
const {
    TOKEN_PROGRAM_ID
} = require('@solana/spl-token');

const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
const simpleProgramId = new PublicKey('BbYJTNjp7BHMdBcy83kVWCbagB2iN26ynKphPK5DZFxG');

const userSecretKey = Uint8Array.from(JSON.parse(fs.readFileSync("/home/seb/MY/TEST_KEYS/user.json", "utf-8")));
const userKeypair = Keypair.fromSecretKey(userSecretKey);

const seedPercentTracker = Buffer.from('percent_tracker');
const [percentTrackerPdaPubkey] = PublicKey.findProgramAddressSync([seedPercentTracker], simpleProgramId);

const seedWsolBalance = Buffer.from('wsol_balance');
const [wsolBalancePdaPubkey] = PublicKey.findProgramAddressSync([seedWsolBalance], simpleProgramId);

const seedTransferAuthority = Buffer.from('transfer_authority');
const [transferAuthorityPdaPubkey] = PublicKey.findProgramAddressSync([seedTransferAuthority], simpleProgramId);

const seedProgramSimpleTokenAccount = Buffer.from('program_simple_token_account');
const [programSimpleTokenAccountPdaPubkey] = PublicKey.findProgramAddressSync([seedProgramSimpleTokenAccount], simpleProgramId);

const [userClaimTrackerPubkey] = PublicKey.findProgramAddressSync([userKeypair.publicKey.toBuffer()], simpleProgramId);

const userSimpleTokenAccountPubkey = new PublicKey('J7wjk7Rt1bcS9STsBbiquk4RB3yqmLnBQeDfs96437k8');
const userRaydiumLpAtaPubkey = new PublicKey('ABfThCNFB8PoD5mK6bPGTKRgxHnfscf7HGWMPVeZBZeZ');

const raydiumPoolWsolTokenAccountPubkey = new PublicKey('6XY1UXw8i4ZydPL4B4Wid2kNDJ7XTDYmFx7Q9ULCUYFJ');

const creatorSimpleTokenAccountPubkey = new PublicKey('8MQDmMgGFV3cA2LBtep1zwh5Sftyk6pw7yzgRjHc4okM');

const simpleMint = new PublicKey('BKPHSeJ4DmQnnT7NwoqirjJaM6GVyxJJyYoHd7TA4hsD');

const raydiumLpMint = new PublicKey('B9F82waRKg622E9FJN9fLN43TNDctGEAqKvCpWvccgvB');

const executeDiscriminator = Buffer.from([130, 221, 242, 154, 13, 193, 189, 29]);

const additionalComputeBudgetInstruction =
    ComputeBudgetProgram.setComputeUnitLimit({
        units: 1_400_000,
});

const executeIx = new TransactionInstruction({
    programId: simpleProgramId,
    keys: [
        { pubkey: userKeypair.publicKey, isSigner: true, isWritable: true },
        { pubkey: percentTrackerPdaPubkey, isSigner: false, isWritable: true },
        { pubkey: wsolBalancePdaPubkey, isSigner: false, isWritable: true },
        { pubkey: transferAuthorityPdaPubkey, isSigner: false, isWritable: false },
        { pubkey: programSimpleTokenAccountPdaPubkey, isSigner: false, isWritable: true },
        { pubkey: userClaimTrackerPubkey, isSigner: false, isWritable: true },
        { pubkey: userSimpleTokenAccountPubkey, isSigner: false, isWritable: true },
        { pubkey: userRaydiumLpAtaPubkey, isSigner: false, isWritable: false },
        { pubkey: raydiumPoolWsolTokenAccountPubkey, isSigner: false, isWritable: false },
        { pubkey: creatorSimpleTokenAccountPubkey, isSigner: false, isWritable: true },
        { pubkey: simpleMint, isSigner: false, isWritable: false },
        { pubkey: raydiumLpMint, isSigner: false, isWritable: false },
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    ],
    data: executeDiscriminator,
});

(async () => {
    try {
        const tx = new Transaction().add(executeIx).add(additionalComputeBudgetInstruction);
        const sig = await sendAndConfirmTransaction(connection, tx, [userKeypair]);
        console.log('Transaction Signature: ', sig);

    } catch (error) {
        console.error('Error: ', error)
    }
})();