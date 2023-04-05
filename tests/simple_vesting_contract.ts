import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SimpleVestingContract } from "../target/types/simple_vesting_contract";

describe("simple_vesting_contract", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SimpleVestingContract as Program<SimpleVestingContract>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
