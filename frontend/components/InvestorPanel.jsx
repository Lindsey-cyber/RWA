import React, { useState } from "react";
import * as SorobanClient from "soroban-client";

const CONTRACT_ID = "CAIUMAVGQUDLA5EMTCC4GY5EF64VMZOFPSS6EFZZKLFWMAB56ZPE5QRP";
const RPC_URL = "https://rpc-futurenet.stellar.org"; // 根据你的网络替换

export default function InvestorPanel() {
  const [amount, setAmount] = useState("");
  const [tranche, setTranche] = useState(null);
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState("");

  async function connectWallet() {
    if (!window.freighterApi) {
      alert("请安装 Freighter 钱包扩展！");
      return null;
    }
    const pubkey = await window.freighterApi.getPublicKey();
    return pubkey;
  }

  async function deposit() {
    if (!amount || !tranche) return alert("请输入金额并选择类型");

    const user = await connectWallet();
    if (!user) return;

    try {
      setLoading(true);
      const server = new SorobanClient.Server(RPC_URL, { allowHttp: true });
      const account = await server.getAccount(user);

      const tx = new SorobanClient.TransactionBuilder(account, {
        fee: "100",
        networkPassphrase: SorobanClient.Networks.FUTURENET,
      })
        .addOperation(
          SorobanClient.Operation.invokeContractFunction({
            contract: CONTRACT_ID,
            function: "subscribe",
            args: SorobanClient.xdr.ScVal.scvVec([
              SorobanClient.Address.fromString(user).toScVal(),
              SorobanClient.xdr.ScVal.scvSymbol(tranche),
              SorobanClient.xdr.ScVal.scvI128(SorobanClient.xdr.Int128Parts.fromString(amount))
            ]),
          })
        )
        .setTimeout(30)
        .build();

      const signedTx = await window.freighterApi.signTransaction(tx.toXDR(), {
        networkPassphrase: SorobanClient.Networks.FUTURENET,
      });

      const txResult = await server.sendTransaction(SorobanClient.TransactionBuilder.fromXDR(signedTx, SorobanClient.Networks.FUTURENET));
      setMessage(`交易已提交: ${txResult.hash}`);
    } catch (err) {
      console.error(err);
      setMessage("交易失败：" + err.message);
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="flex flex-col gap-4 w-80 bg-white p-6 rounded shadow">
      <h2 className="text-xl font-bold">投资者操作</h2>
      <div className="flex gap-2">
        <button
          onClick={() => setTranche("Junior")}
          className={`flex-1 px-4 py-2 rounded ${tranche === "Junior" ? "bg-green-500 text-white" : "bg-gray-200"}`}
        >
          Junior
        </button>
        <button
          onClick={() => setTranche("Senior")}
          className={`flex-1 px-4 py-2 rounded ${tranche === "Senior" ? "bg-blue-500 text-white" : "bg-gray-200"}`}
        >
          Senior
        </button>
      </div>

      {tranche && (
        <div className="flex flex-col gap-2">
          <input
            type="number"
            placeholder="输入存入数量"
            className="border p-2 rounded"
            value={amount}
            onChange={(e) => setAmount(e.target.value)}
          />
          <button
            onClick={deposit}
            disabled={loading}
            className="bg-indigo-600 text-white py-2 rounded"
          >
            {loading ? "处理中..." : "确认存入"}
          </button>
        </div>
      )}
      {message && <p className="text-sm text-gray-600">{message}</p>}
    </div>
  );
}
