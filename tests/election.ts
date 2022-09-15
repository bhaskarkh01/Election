import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Election } from "../target/types/election";

describe("election", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Election as Program<Election>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
