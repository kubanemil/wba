import wallet from "./wallet/wba-wallet.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { 
    findMetadataPda,
    updateMetadataAccountV2,
    UpdateMetadataAccountV2InstructionAccounts,
    UpdateMetadataAccountV2InstructionArgs,
    DataV2Args
} from "@metaplex-foundation/mpl-token-metadata";
import { createSignerFromKeypair, signerIdentity, publicKey } from "@metaplex-foundation/umi";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";


// Define our Mint address
const mintId = publicKey("ECHspzrpHhL1ofEMnFuEarrs3frGJe2tdxeuWjww2ZsL")

// Create a UMI connection
const umi = createUmi('https://api.devnet.solana.com');
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(signer));


(async () => {
    try {
        let metadataId = findMetadataPda(umi, {mint: mintId});

        // Start here
        let accounts: UpdateMetadataAccountV2InstructionAccounts = {
            metadata: metadataId
        }

        // can update data
        let data: DataV2Args = {
            name: "EmilLolToken",
            symbol: "ELolToken",
            // don't forget to change name, symbol and image in metadata URI - they are not the same!
            uri: "https://turquoise-used-shark-708.mypinata.cloud/ipfs/QmbW8HunnGGccHBCfWvchb7GMWwwPqmAtxnu4Hw7JAvJpc", // metadata URI
            sellerFeeBasisPoints: 10,
            creators: null,
            collection: null,
            uses: null
        }

        let args: UpdateMetadataAccountV2InstructionArgs = {
            data
        }

        let updateMetdataTX = updateMetadataAccountV2(
            umi,
            {
                ...accounts,
                ...args
            }
        )

        let result = await updateMetdataTX.sendAndConfirm(umi);
        console.log(bs58.encode(result.signature));
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();
