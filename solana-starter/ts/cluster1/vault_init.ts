import {
  Connection,
  Keypair,
  SystemProgram,
  PublicKey,
  Commitment,
} from "@solana/web3.js";
import { Program, Wallet, AnchorProvider, Address } from "@coral-xyz/anchor";
import { WbaVault, IDL } from "./programs/wba_vault";
import wallet from "./wallet/wba-wallet.json";

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// Commitment
const commitment: Commitment = "confirmed";

// Create a devnet connection
const connection = new Connection("https://api.devnet.solana.com");

// Create our anchor provider
const provider = new AnchorProvider(connection, new Wallet(keypair), {
  commitment,
});

// Create our program
const program = new Program<WbaVault>(
  IDL,
  "D51uEDHLbWAxNfodfQDv7qkp8WZtxrhi3uganGbNos7o" as Address,
  provider,
);

// Create a random keypair
const vaultState = Keypair.generate();
console.log(`Vault public key: ${vaultState.publicKey.toBase58()}`); // A7GTzaUzcUMEPnUK4ptM97MnDJEM7N4dNVC2NkNKPWsd

// Create the PDA for our enrollment account
// Seeds are "auth", vaultState
const vaultAuth = PublicKey.findProgramAddressSync([Buffer.from("auth"), vaultState.publicKey.toBuffer()], program.programId)[0];

// Create the vault key
// Seeds are "vault", vaultAuth
const vault = PublicKey.findProgramAddressSync([Buffer.from("vault"), vaultAuth.toBuffer()], program.programId)[0];

// Execute our enrollment transaction
(async () => {
  try {
    const signature = await program.methods.initialize()
    .accounts({
        owner: keypair.publicKey,
        vaultState: vaultState.publicKey,
        vaultAuth: vaultAuth,
        vault: vault,
        // systemProgram: SystemProgram.programId,
        
    }).signers([keypair, vaultState]).rpc(); // 4L1rKEjTAJckv7PWp6p1MewoB5JGVgvwmyFWJe5ud5mzvkWwYCpW1Av4kM7azZMZeKyhrz7EhgtJLV9Ch6q1sbd6
    console.log(`Init success! Check out your TX here: \nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
