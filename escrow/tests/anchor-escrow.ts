import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";

describe("anchor-escrow", () => {
    // Configure the local cluster
    anchor.setProvider(anchor.AnchorProvider.env());

    const provider = anchor.getProvider();
    const connection = provider.connection;

    const program = anchor.workspace.escrow as Program<Escrow>;
    it("Initialize the program", async () => {
        const id=new BN(12), tokenA_offered_amount=new BN(12), token_B_desired_amount = new BN(12);
        const tx = await program.methods
            .makeOffer(id, tokenA_offered_amount, token_B_desired_amount)
            // .accounts({})
            // .signers([])
            .rpc();
        console.log(tx);
    })
})