"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const web3_js_1 = require("@solana/web3.js");
const fs_1 = __importDefault(require("fs"));
let programId = new web3_js_1.PublicKey("24x6XDgxxZgSzuAefWmx7WAppzBfgCSHtxAkDtpALbq1");
const connection = new web3_js_1.Connection((0, web3_js_1.clusterApiUrl)('devnet'), 'confirmed');
const secretKey = JSON.parse(fs_1.default.readFileSync('../../KEYS/simple.json', 'utf8'));
// Create a Keypair from the secret key
const payer = web3_js_1.Keypair.fromSecretKey(new Uint8Array(secretKey));
main()
    .then(() => {
    console.log("the numbers mason");
})
    .catch((error) => {
    console.log(error);
});
function main() {
    return __awaiter(this, void 0, void 0, function* () {
        let payer = web3_js_1.Keypair.fromSecretKey(new Uint8Array(secretKey));
        const tx = yield runProgram(payer);
        console.log(tx);
    });
}
function runProgram(payer) {
    return __awaiter(this, void 0, void 0, function* () {
        const tx = new web3_js_1.Transaction();
        const instruction = new web3_js_1.TransactionInstruction({
            keys: [],
            programId,
        });
        tx.add(instruction);
        const txSig = yield (0, web3_js_1.sendAndConfirmTransaction)(connection, tx, [payer]);
        return txSig;
    });
}
