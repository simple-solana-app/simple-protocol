const {
    TransactionInstruction,
    Transaction,
    sendAndConfirmTransaction,
    ComputeBudgetProgram
} = require('@solana/web3.js');
const {
    TOKEN_PROGRAM_ID
} = require('@solana/spl-token');
const {
    connection,
    simpleProtocolProgramId,
    userKeypair,
    percentTrackerPdaPubkey,
    wsolBalancePdaPubkey,
    transferAuthorityPdaPubkey,
    programSimpleTokenAccountPdaPubkey,
    userClaimTrackerPubkey,
    userSimpleTokenAccountPubkey,
    userRaydiumLpAtaPubkey,
    raydiumPoolWsolTokenAccountPubkey,
    creatorSimpleTokenAccountPubkey,
    simpleProtocolTokenMint,
    raydiumLpMint,
    user1Keypair,
    user1ClaimTrackerPubkey,
    user1SimpleTokenAccountPubkey,
    user1RaydiumLpAtaPubkey
} = require('../directory')

const executeDiscriminator = Buffer.from([130, 221, 242, 154, 13, 193, 189, 29]);

const additionalComputeBudgetInstruction =
    ComputeBudgetProgram.setComputeUnitLimit({
        units: 1_400_000,
});

const executeIx = new TransactionInstruction({
    programId: simpleProtocolProgramId,
    keys: [
        { pubkey: user1Keypair.publicKey, isSigner: true, isWritable: true },
        { pubkey: percentTrackerPdaPubkey, isSigner: false, isWritable: true },
        { pubkey: wsolBalancePdaPubkey, isSigner: false, isWritable: true },
        { pubkey: transferAuthorityPdaPubkey, isSigner: false, isWritable: false },
        { pubkey: programSimpleTokenAccountPdaPubkey, isSigner: false, isWritable: true },
        { pubkey: user1ClaimTrackerPubkey, isSigner: false, isWritable: true },
        { pubkey: user1SimpleTokenAccountPubkey, isSigner: false, isWritable: true },
        { pubkey: user1RaydiumLpAtaPubkey, isSigner: false, isWritable: false },
        { pubkey: raydiumPoolWsolTokenAccountPubkey, isSigner: false, isWritable: false },
        { pubkey: creatorSimpleTokenAccountPubkey, isSigner: false, isWritable: true },
        { pubkey: simpleProtocolTokenMint, isSigner: false, isWritable: false },
        { pubkey: raydiumLpMint, isSigner: false, isWritable: false },
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    ],
    data: executeDiscriminator,
});

(async () => {
    try {
        const tx = new Transaction().add(executeIx).add(additionalComputeBudgetInstruction);
        const sig = await sendAndConfirmTransaction(connection, tx, [user1Keypair]);
        console.log(sig);

    } catch (error) {
        console.error(error)
    }
})();