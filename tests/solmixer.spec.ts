import * as anchor from "@project-serum/anchor";
import * as assert from "assert";

describe("solmixer", () => {
  let provider = anchor.Provider.env();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  const program = anchor.workspace.Solmixer;

  it("Is initialized!", async () => {
    await program.state.rpc.new({
      accounts: {
        authority: provider.wallet.publicKey,
      },
    });
    const management = await program.state();
    assert.ok(management.authority.equals(provider.wallet.publicKey));
  });

  let depositQ = new anchor.web3.Account();
  let laundromat = new anchor.web3.Account();

  it("creates a laundromat", async () => {
    await program.rpc.newLaundromat(new anchor.BN(0), {
      accounts: {
        laundromat: laundromat.publicKey,
        depositQ: depositQ.publicKey,
        authority: provider.wallet.publicKey,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      },
      instructions: [
        await program.account.depositQ.createInstruction(depositQ),
        await program.account.laundromat.createInstruction(laundromat),
      ],
      signers: [depositQ, laundromat],
    });
    let laundromatAcct = await program.account.laundromat(laundromat.publicKey);
    assert.ok(laundromatAcct.totalWashedFunds.eq(new anchor.BN(0)))
    assert.ok(laundromatAcct.totalUnwashedFunds.eq(new anchor.BN(0)));
    assert.ok(laundromatAcct.asset.eq(new anchor.BN(0)));
  });
  it("deposits into laundromat", async () => {
    await program.rpc.depositIntoLaundromat(new anchor.BN(100), {
      accounts: {
        laundromat: laundromat.publicKey,
        depositQ: depositQ.publicKey,
        authority: provider.wallet.publicKey,
      },
      lamports: new anchor.BN(100),
    });
    let laundro = await program.account.laundromat(laundromat.publicKey);
    assert.ok(laundro.totalUnwashedFunds.eq(new anchor.BN(100)));
    let depositq = await program.account.depositQ(depositQ.publicKey);
    assert.ok(depositq.numDeposits.eq(new anchor.BN(1)));
    assert.ok(depositq.deposits[0].from.equals(provider.wallet.publicKey));
    assert.ok(depositq.deposits[0].amount.eq(new anchor.BN(100)));
  })
  it("tumbles laundromat", async () => {
    await program.rpc.tumbleLaundromat({
      accounts: {
        laundromat: laundromat.publicKey,
        depositQ: depositQ.publicKey,
        authority: provider.wallet.publicKey,
      },
    })
  })
});
