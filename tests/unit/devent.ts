import * as anchor from "@project-serum/anchor";
import { assert } from "chai";
import { Devent } from "../../target/types/devent";

describe("devent unit tests", () => {
  const provider = anchor.AnchorProvider.env();
  const wallet = provider.wallet as anchor.Wallet;
  anchor.setProvider(provider);
  const deventProgram = anchor.workspace.Devent as anchor.Program<Devent>;

  describe("create_state()", () => {
    const stateKeyPair: anchor.web3.Keypair = anchor.web3.Keypair.generate();
    it("initializes event_count correctly", async () => {
      const tx = await deventProgram.methods
        .createState()
        .accounts({
          state: stateKeyPair.publicKey,
          authority: wallet.publicKey,
        })
        .signers([stateKeyPair])
        .rpc();
      const stateData = await deventProgram.account.stateAccount.fetch(
        stateKeyPair.publicKey
      );
      const zero = new anchor.BN(0);
      assert.equal(stateData.eventCount.toString(), zero.toString());
    });
  });

  describe("create_event()", () => {
    it("throws an exception if StateAccount is not created", async () => {});
    it("assigns data correctly", async () => {});
  });
  describe("attendee_registers()", () => {
    it("does not transfer lamports if Pubkey is already registered", async () => {});
  });
});
