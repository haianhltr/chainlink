import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { ChainlinkDapp } from "../target/types/chainlink_dapp";

describe("chainlink_dapp", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ChainlinkDapp as Program<ChainlinkDapp>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
