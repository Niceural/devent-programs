import * as anchor from "@project-serum/anchor";
import { Devent } from "../../target/types/devent";

describe("Integration tests: creating an event", () => {
  const provider = anchor.AnchorProvider.env();
  const wallet = provider.wallet as anchor.Wallet;
  anchor.setProvider(provider);
  const program = anchor.workspace.Devent as anchor.Program<Devent>;
});
