import {
    Horizon,
    Asset,
    Keypair,
    TransactionBuilder,
    Networks,
    BASE_FEE,
    Operation,
  } from "@stellar/stellar-sdk";
  
  const USDC_ISSUER = "GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5";
  const USDC_ASSET = new Asset("USDC", USDC_ISSUER);
  
  const horizonServer = new Horizon.Server("https://horizon-testnet.stellar.org");
  
  const keypair = Keypair.fromSecret("SAQTAP6M6RFEEK5AKYQ5NEQOFZIGVLZVZB5XBOH2V7Z3DUY5MH372QZQ");
  
  (async () => {
    const account = await horizonServer.loadAccount(keypair.publicKey());
  
    const tx = new TransactionBuilder(account, {
      fee: BASE_FEE,
      networkPassphrase: Networks.TESTNET,
    })
      .addOperation(
        Operation.changeTrust({
          asset: USDC_ASSET,
          limit: "100000000000", // 设置信任额度
        })
      )
      .setTimeout(30)
      .build();
  
    tx.sign(keypair);
    const result = await horizonServer.submitTransaction(tx);
    console.log("✅ Trustline created:", result.hash);
  })();
  