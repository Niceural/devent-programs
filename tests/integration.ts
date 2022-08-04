import * as anchor from "@project-serum/anchor";
import { Devent } from "../target/types/devent";

describe("devent integration tests", () => {
  const provider = anchor.AnchorProvider.env();
  const wallet = provider.wallet as anchor.Wallet;
  anchor.setProvider(provider);
  const program = anchor.workspace.Devent as anchor.Program<Devent>;

  const metadataUrl =
    "https://raw.githubusercontent.com/Niceural/devent-programs/main/assets/testEvent1.json";
  const registrationLimit = new anchor.BN(119);
  const minLamportsPrice = new anchor.BN(8700000000);

  it("creates events", async () => {
    let [stateSigner] = await anchor.web3.PublicKey.findProgramAddress(
      [anchor.utils.bytes.utf8.encode("state")],
      program.programId
    );
    let stateInfo;
    try {
      stateInfo = await program.account.st``.fetch(stateSigner);
    } catch (error) {
      console.log(error);
      await program.methods
        .createState()
        .accounts({ state: stateSigner.publicKey, authority: wallet.publicKey })
        .signers([stateSigner])
        .rpc();
      return false;
    }

    console.log("Getting event signer...");
    let [eventSigner] = await anchor.web3.PublicKey.findProgramAddress([
      anchor.utils.bytes.utf8.encode("event"),
      stateInfo.eventCount.toArrayLike(Buffer, "be", 8),
    ]);
    try {
      await program.account.eventAccount.fetch(eventSigner);
    } catch (error) {
      console.log(error);
      await program.methods
        .createEvent(metadataUrl, registrationLimit, minLamportsPrice)
        .accounts({
          state: stateSigner.publicKey,
          event: eventSigner.publicKey,
          authority: wallet.publicKey,
        })
        .signers([eventSigner])
        .rpc();
    }
  });
});
