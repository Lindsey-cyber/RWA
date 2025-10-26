// scripts/supply-to-pool.js
// 直接调用 Soroban token contract 的 `transfer` entrypoint，把 token 转给 pool 合约地址。
// 使用方法：在项目根执行 `node scripts/supply-to-pool.js`
// 依赖： 全局可用 `stellar` CLI 的环境变量或在此脚本通过 child_process 调用 `stellar contract invoke`。
// 我这里用 child_process 简单包装，你也可以改为用 JS SDK。

const { execSync } = require("child_process");

const TOKEN_CONTRACT_ID = process.env.TOKEN_CONTRACT_ID || "CCA2BWGKIB7TU5VWHZSRDSGQPSIROSHGE4RUXOW4S6RMGU4DK5EXO7BN";
const POOL_CONTRACT_ID = process.env.POOL_CONTRACT_ID || "CD24SABPPEFJHQ4D5UEVAV52SUYHDERKKBNWX2PUGVPSJ6NCOEJVBLTQ";
const SECRET_KEY = process.env.SECRET_KEY || process.env.SECRET || "SC566PQX2UK4X7JXEF7UU7PE5IYCDGPE6C6VQH26RHLYNFKEEAWGSHHV";
const AMOUNT = process.env.AMOUNT || "1000";

if (!TOKEN_CONTRACT_ID || !POOL_CONTRACT_ID || !SECRET_KEY) {
  console.error("请在环境变量中设置 TOKEN_CONTRACT_ID, POOL_CONTRACT_ID, SECRET_KEY");
  process.exit(1);
}

try {
  console.log("开始把 token 转给 pool……");
  const cmd = [
    "stellar",
    "contract",
    "invoke",
    "--network",
    "testnet",
    "--id",
    TOKEN_CONTRACT_ID,
    "--source",
    SECRET_KEY,
    "--",
    "transfer",
    "--to",
    POOL_CONTRACT_ID,
    "--amount",
    AMOUNT
  ].join(" ");
  console.log("执行命令:", cmd);
  const out = execSync(cmd, { stdio: "pipe" }).toString();
  console.log("命令输出:\n", out);
  console.log("现在查询 token 合约对 pool 的余额（read-only）:");
  const balCmd = [
    "stellar",
    "contract",
    "invoke",
    "--network",
    "testnet",
    "--id",
    TOKEN_CONTRACT_ID,
    "--source-account",
    process.env.PUBLIC_KEY || "",
    "--",
    "balance",
    "--id",
    POOL_CONTRACT_ID
  ].join(" ");
  console.log("查询命令:", balCmd);
  const balOut = execSync(balCmd, { stdio: "pipe" }).toString();
  console.log("balance 输出:\n", balOut);
} catch (e) {
  console.error("执行出错：", e.stdout ? e.stdout.toString() : e.message);
  process.exit(1);
}
