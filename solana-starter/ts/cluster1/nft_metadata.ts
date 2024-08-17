import wallet from "./wallet/wba-wallet.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        const imageURL = "https://arweave.net/W3ibdiICmKhtl1E5jO39E5Prnen8qYIHusTzA9lyFnE"
        const metadata = {
            name: "EmilRug",
            symbol: "ERug",
            description: "MyRug NFT",
            image: imageURL,
            attributes: [
                {trait_type: '?', value: '?'}
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: imageURL
                    },
                ]
            },
            creators: []
        };
        const myMetadataURI = await umi.uploader.uploadJson(metadata);
        console.log("Your metadata URI: ", myMetadataURI); // https://arweave.net/peuhn6bKIfWiE_fb0BcIjp7m41cgDAsFcAcBmtdlBJU
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
