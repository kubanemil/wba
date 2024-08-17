import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "./wallet/wba-wallet.json";
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mintId = new PublicKey("ECHspzrpHhL1ofEMnFuEarrs3frGJe2tdxeuWjww2ZsL");

// Recipient address
const to = new PublicKey("E3VnKGyxN3FqwySrRgMYUGaMtG5GjGsSn34WkbX9dmnW");

const token_decimals = 1_000_000n;

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const ata_from = await getOrCreateAssociatedTokenAccount(connection, keypair, mintId, keypair.publicKey);
        // Get the token account of the toWallet address, and if it does not exist, create it
        const ata_to = await getOrCreateAssociatedTokenAccount(connection, keypair, mintId, to);

        // Transfer the new token to the "toTokenAccount" we just created
        let tx = await transfer(connection, keypair, ata_from.address, ata_to.address, keypair, 10n*token_decimals)
        console.log("Transaction signature of token transfer: ", tx);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();