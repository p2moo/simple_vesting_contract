import * as anchor from "@project-serum/anchor";
import {Program} from "@project-serum/anchor";
import {SimpleVestingContract} from "../target/types/simple_vesting_contract";

describe("simple_vesting_contract", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .SimpleVestingContract as Program<SimpleVestingContract>;

  it("Is initialized!", async () => {
    // Create escrow account using 3 different types of seeds.
    // The version seed is important as it allows the same depositor to send to the same recipient in multiple instances.
    const seed = Buffer.from("ChiaSeeds", "utf8");
    const recipient = new anchor.web3.PublicKey(
      "BHDbaJCph8MuhkXfSFi4zrjUTTzEgzTwWmbAbPnmu6ki"
    );
    const version = Buffer.from("Version1", "utf8");
    const [escrowAccount, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [seed, recipient.toBuffer(), version],
      program.programId
    );

    // We are testing to deposit the funds, so we need to create a test depositor.
    // This is not necessary for the actual deposit, since the depositor would already have their own wallet.
    // Create keypair for depositor
    const depositor = anchor.web3.Keypair.generate();
    const connection = anchor.getProvider().connection;
    const signature = await connection.requestAirdrop(
      depositor.publicKey,
      50 * anchor.web3.LAMPORTS_PER_SOL
    );
    const latestBlockHash = await connection.getLatestBlockhash();
    await connection.confirmTransaction(
      {
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature,
      },
      "confirmed"
    );

    // To-do: have the depositor transfer funds to escrow
    // Then call create vesting function
    // For lucas - how to make contract safer so it doesn't complain
  });
});
