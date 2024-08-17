import {
  Connection,
  Keypair,
  SystemProgram,
  PublicKey,
  Commitment,
} from "@solana/web3.js";
import {
  Program,
  Wallet,
  AnchorProvider,
  Address,
  BN,
} from "@coral-xyz/anchor";
import { WbaVault, IDL } from "./programs/wba_vault";
import wallet from "./wallet/wba-wallet.json";

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// Commitment
const commitment: Commitment = "finalized";

// Create a devnet connection
const connection = new Connection("https://api.devnet.solana.com");

// Create our anchor provider
const provider = new AnchorProvider(connection, new Wallet(keypair), {
  commitment,
});

// Create our program
const program = new Program<WbaVault>(IDL, "D51uEDHLbWAxNfodfQDv7qkp8WZtxrhi3uganGbNos7o" as Address, provider);
const vaultState = new PublicKey("A7GTzaUzcUMEPnUK4ptM97MnDJEM7N4dNVC2NkNKPWsd");
const vaultAuth = PublicKey.findProgramAddressSync([Buffer.from("auth"), vaultState.toBuffer()], program.programId)[0];
const vault = PublicKey.findProgramAddressSync([Buffer.from("vault"), vaultAuth.toBuffer()], program.programId)[0];

// Execute our enrollment transaction
(async () => {
  try {
    const signature = await program.methods
      .deposit(new BN(1000_000_000)) // 1 SOL
      .accounts({
        owner: keypair.publicKey,
        vaultState: vaultState,
        vaultAuth: vaultAuth,
        vault: vault,
      })
      .signers([
        keypair
      ]).rpc(); // 4UGYPvjKcEQahktxea9qNoc3c82cFAeU2BgDmoAUoaUGuWzgrpxRKGAoY2cpXbBxuPHQRtNBB8oZLLUMe9tcUVwL
    console.log(`Deposit success! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
