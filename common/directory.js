const fs = require('fs');
const {
    Connection,
    PublicKey,
    Keypair
} = require('@solana/web3.js');

const connection = new Connection('https://api.mainnet-beta.solana.com', 'confirmed');

const simpleProtocolProgramId = new PublicKey('6yUtbQXotEAbzJBHLghordn9r3vZ8wRuCbFBxMaatVoF');

// token mints
const simpleProtocolTokenMint = new PublicKey('4QUwG4eADsjfaZ5nTEd6eGF5he8vR8FCFLPgwmpiJRD5');
const raydiumLpMint = new PublicKey('52Pbw9eUXkuMsw1KJKdYtkBEPt94D8RL8Ko29Hrqsb2X');
const wsolMint = new PublicKey('6u9PpjPzYgzeg2iSLb3iNox14dnt6ztr7W7g2fKjfFCA');

// individuals
const simplePubkey = new PublicKey('E61fUAd1cxFES9kPckPhzwiiFMRo8ezAw7ZG5a8YD2jv');
const simpleSecretKey = Uint8Array.from(JSON.parse(fs.readFileSync("/home/seb/MY/KEYS/simple.json", "utf-8")));
const simpleKeypair = Keypair.fromSecretKey(simpleSecretKey);
const simpleRaydiumLpTokenAccountPubkey = new PublicKey('FMfEFEH98S6NzRR7YxNJ2jXVM9ZahkcSMjmcvKiYnEaV');

const mySimPubkey = new PublicKey('2ynbpLvcfzC6oTPnZ76nBzisVWoBFwU5iUh2s2F56Gd7');
const mySimSecretKey = Uint8Array.from(JSON.parse(fs.readFileSync("/home/seb/MY/TEST_KEYS/my_sim.json", "utf-8")));

const userPubkey = new PublicKey('92uCjKrA55uFXs2d8padAe57aAnbnDJtVXurP4yTxapY');
const userSecretKey = Uint8Array.from(JSON.parse(fs.readFileSync("/home/seb/MY/TEST_KEYS/user.json", "utf-8")));
const userKeypair = Keypair.fromSecretKey(userSecretKey);
const [userClaimTrackerPubkey] = PublicKey.findProgramAddressSync([userKeypair.publicKey.toBuffer()], simpleProtocolProgramId);
const userSimpleTokenAccountPubkey = new PublicKey('J7wjk7Rt1bcS9STsBbiquk4RB3yqmLnBQeDfs96437k8');
const userRaydiumLpAtaPubkey = new PublicKey('ABfThCNFB8PoD5mK6bPGTKRgxHnfscf7HGWMPVeZBZeZ');

const user1Pubkey = new PublicKey('34t4WSTs9VnNxBgpW6UPU8hWvfGCQ4Bh9A93LRBcGyH9');
const user1SecretKey = Uint8Array.from(JSON.parse(fs.readFileSync("/home/seb/MY/TEST_KEYS/user_1.json", "utf-8")));
const user1Keypair = Keypair.fromSecretKey(user1SecretKey);
const [user1ClaimTrackerPubkey] = PublicKey.findProgramAddressSync([user1Keypair.publicKey.toBuffer()], simpleProtocolProgramId);
const user1SimpleTokenAccountPubkey = new PublicKey('2KHpEZKnzkQyabgXGCwSxd8NTBZwEmPQPYpYGk2yKKVA');
const user1RaydiumLpAtaPubkey = new PublicKey('EFoLQJsb3NaZPcrfS8w2sKCPfSGvAYLUktQaJJDgGHoR');

const creatorPubkey = new PublicKey('GBg4zyaUHtAaBXasgPPmb45hQkK6NgayzdfmeeiYmtGd');
const creatorSimpleTokenAccountPubkey = new PublicKey('5LEXeqv44X21oCBybV74ZTQCVKLtX1iL5474gSUjWwrx');

// accts
const seedPercentTracker = Buffer.from('percent_tracker');
const [percentTrackerPdaPubkey] = PublicKey.findProgramAddressSync([seedPercentTracker], simpleProtocolProgramId);
const seedWsolBalance = Buffer.from('wsol_balance');
const [wsolBalancePdaPubkey] = PublicKey.findProgramAddressSync([seedWsolBalance], simpleProtocolProgramId);
const seedTransferAuthority = Buffer.from('transfer_authority');
const [transferAuthorityPdaPubkey] = PublicKey.findProgramAddressSync([seedTransferAuthority], simpleProtocolProgramId);
const seedProgramSimpleTokenAccount = Buffer.from('program_simple_token_account');
const [programSimpleTokenAccountPdaPubkey] = PublicKey.findProgramAddressSync([seedProgramSimpleTokenAccount], simpleProtocolProgramId);

const raydiumPoolWsolTokenAccountPubkey = new PublicKey('364AQ7xZsUn3R9qkYSDVks1W6pfiXzZosJjZ6o7gv9by');

module.exports = {
    connection,
    simpleProtocolProgramId,
    simpleKeypair,
    simpleProtocolTokenMint,
    percentTrackerPdaPubkey,
    wsolBalancePdaPubkey,
    transferAuthorityPdaPubkey,
    programSimpleTokenAccountPdaPubkey,
    userKeypair,
    userClaimTrackerPubkey,
    userSimpleTokenAccountPubkey,
    userRaydiumLpAtaPubkey,
    raydiumPoolWsolTokenAccountPubkey,
    creatorSimpleTokenAccountPubkey,
    raydiumLpMint,
    user1Keypair,
    user1ClaimTrackerPubkey,
    user1SimpleTokenAccountPubkey,
    user1RaydiumLpAtaPubkey
}