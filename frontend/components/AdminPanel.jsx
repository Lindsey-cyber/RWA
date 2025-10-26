import React, { useState } from "react";
import * as SorobanClient from "soroban-client";

const CONTRACT_ID = "CAIUMAVGQUDLA5EMTCC4GY5EF64VMZOFPSS6EFZZKLFWMAB56ZPE5QRP";
const RPC_URL = "https://rpc-futurenet.stellar.org";

export default function AdminPanel() {
  const [amount, setAmount] = useState("");
  const [message, setMessage] = useState("");

  async function connectWallet() {
    if (!window.freighterApi) {
      alert("请安装 Freighter 钱包扩展！");
      return null;
    }
    const pubkey = await window.freighterApi.getPublicKey();
    return pubkey;
  }

  async function callAdminFunction(func, args = []) {
    const admin = await connectWallet();
    if (!admin) return;

    const server = new SorobanClient.Server(RPC_URL, { allowHttp: true });
    const account = await server.getAccount(admin);

    const tx = new SorobanClient.TransactionBuilder(account, {
      fee: "100",
      networkPassphrase: SorobanClient.Networks.FUTURENET,
    })
      .addOperation(
        SorobanClient.Operation.invokeContractFunction({
          contract: CONTRACT_ID,
          function: func,
          args,
        })
      )
      .setTimeout(30)
      .build();

    const signedTx = await window.freighterApi.signTransaction(tx.toXDR(), {
      networkPassphrase: SorobanClient.Networks.FUTURENET,
    });
    const txResult = await server.sendTransaction(
      SorobanClient.TransactionBuilder.fromXDR(signedTx, SorobanClient.Networks.FUTURENET)
    );
    setMessage(`已调用 ${func}: ${txResult.hash}`);
  }

  async function simulateRWA() {
    if (!amount) return alert("请输入收益金额");
    await callAdminFunction("notify_pool_payout", [
      SorobanClient.Address.fromString(await connectWallet()).toScVal(),
      SorobanClient.xdr.ScVal.scvI128(SorobanClient.xdr.Int128Parts.fromString(amount))
    ]);
  }

  async function simulateLoss() {
    if (!amount) return alert("请输入损失金额");
    await callAdminFunction("apply_loss", [
      SorobanClient.Address.fromString(await connectWallet()).toScVal(),
      SorobanClient.xdr.ScVal.scvI128(SorobanClient.xdr.Int128Parts.fromString(amount))
    ]);
  }

  return (
    <div className="flex flex-col gap-4 w-80 bg-white p-6 rounded shadow">
      <h2 className="text-xl font-bold">管理员操作</h2>
      <input
        type="number"
        placeholder="输入金额"
        className="border p-2 rounded"
        value={amount}
        onChange={(e) => setAmount(e.target.value)}
      />
      <button onClick={simulateRWA} className="bg-green-600 text-white py-2 rounded">
        模拟RWA收益分配
      </button>
      <button onClick={simulateLoss} className="bg-red-600 text-white py-2 rounded">
        模拟亏损分配
      </button>
      {message && <p className="text-sm text-gray-600">{message}</p>}
    </div>
  );
}
