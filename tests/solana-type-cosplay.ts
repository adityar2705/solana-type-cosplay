import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaTypeCosplay } from "../target/types/solana_type_cosplay";

describe("solana-type-cosplay", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  //get an instance of the type cosplay program
  const program = anchor.workspace.SolanaTypeCosplay as Program<SolanaTypeCosplay>;

  const userAccount = anchor.web3.Keypair.generate();
  const newAdmin = anchor.web3.Keypair.generate();

  it("initializes user account", async () => {
    //initialize the user account
    const tx = await program.methods
    .initializeUser()
    .accounts({
      payer: provider.wallet.publicKey,
      newAccount:userAccount.publicKey
    })
    .signers([userAccount])
    .rpc();

  });

  it("updates the new admin using the user account", async () => {
    //this is not secure as even a user account type is able to alter the admin
    const tx = await program.methods
    .updateAdmin()
    .accounts({
      adminConfig: userAccount.publicKey,
      newAdmin:newAdmin.publicKey
    })
    .rpc();

    //unless we use discriminators -> the Solana program will just see the order of the accounts and allow execution -> which can be very insecure
    console.log("✅ Transaction was successful!");
  });
});
