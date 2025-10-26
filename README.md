# 🌐 RWA Tranche — Risk-Adjusted Yield for On-Chain Real-World Assets

> **RWA Tranche** brings structured yield tranching (Senior / Junior layers) to tokenized Real-World Assets (RWAs) on **Stellar**, enabling **risk-adjusted, transparent, and automated yield distribution** through Soroban smart contracts.

video link: https://drive.google.com/drive/folders/1CsJ5002g4GsyauiBjsTzUvHHfgNFm8Vd?usp=drive_link  
you could run it locally with npm run dev easily on localhost:3000 with https://github.com/Lindsey-cyber/stellar (our very pretty frontend deployment)
---
## 🚀 Overview

Real-World Assets (RWAs) such as tokenized Treasury bills are rapidly growing on Stellar — yet most of them are **static holdings** that lack yield differentiation or risk management mechanisms.

In traditional finance, **tranching** redistributes yield and risk:  
- **Senior tranches** earn stable returns with protection.  
- **Junior tranches** take higher risk for higher potential yield.

**RWA Tranche** brings this mechanism on-chain using Soroban smart contracts, creating a composable and transparent DeFi primitive for structured yield products.

---## 🔍 Background Concepts

### 🏦 Real-World Assets (RWAs) on Stellar
**RWAs** are off-chain financial or physical assets — e.g., Treasury bills, corporate bonds, or cash-flow streams — **represented on-chain as tokenized claims**.  

Stellar advantages for RWAs:
- Native asset issuance and trustline model  
- Fast, low-cost transfers  
- Programmable contracts with Soroban  

In this project, **RWA tokens** act as the yield-generating base for tranches. Periodic interest or revenue flows from the RWA pool into the tranche contracts.

### 🧮 Tranches in Traditional Finance
A **tranche** (“slice”) segments a pool of assets into layers with **different risk-return and repayment priorities**:  
- **Senior** → priority yield, protected from losses; lower risk/reward  
- **Junior** → absorbs losses first; higher risk/reward  
- (Optional) **Mezzanine** → intermediate risk/reward  

**On-chain adaptation:**  
- Senior/Junior tokens minted by the Soroban contract  
- Yield allocation prioritizes Senior tranche  
- Losses first absorbed by Junior tranche  

This mirrors traditional finance mechanics in a fully decentralized, programmable environment.

---

## 🧩 How It Works

1. **RWA Tokenization:**  
   - Issue tokenized RWAs (e.g., RWA_UST) on Stellar Testnet via Soroban token contract.  
   - Mint tokens to issuer/custodian accounts.

2. **Blend Pool Integration:**  
   - Deposit RWA tokens into a **Blend pool** to simulate yield generation.  
   - Pool emits payouts triggered manually (demo) or via oracle (production).

3. **Tranche Contract Deployment (Soroban):**  
   - Mint **Senior** and **Junior tranche tokens**.  
   - Users subscribe with XLM/USDC.  
   - On pool payout:  
     - Senior tranche receives yield first  
     - Junior receives remaining  
   - On losses: Junior absorbs first, Senior protected.

4. **User Interaction:**  
   - Subscribe, redeem, or switch tranches via smart contract calls.  
   - Admin can pause/resume, update minimums, or trigger payouts.

---

## 💡 Why We Built It

- Current RWA tokens on Stellar offer **flat, undifferentiated returns**.
- There’s **no way for users to express risk appetite** or build layered yield strategies.
- We wanted to make RWAs not just exist on-chain — but **work** on-chain.

Our goal:  
> Transform RWAs from passive tokens into **dynamic yield-generating building blocks** for the Stellar DeFi ecosystem.

---

## ⚙️ How It Works

### 🎯 Core Architecture

RWA Tranche uses **Soroban smart contracts** integrated with **Blend Pool** to create structured products with automatic yield distribution.

| Component | Description |
|------------|-------------|
| **Tranche Contract** | Core logic for Senior / Junior token issuance, yield allocation, and loss absorption. |
| **Underlying Pool (Blend Pool)** | Simulates the yield source (e.g., Treasury returns). |
| **Benefit Simulator** | Injects simulated periodic yield for testing and demo. |

### 💵 Yield & Risk Logic

- **Senior Tranche** → Receives yield first, bears losses last (low-risk, stable return).  
- **Junior Tranche** → Receives yield last, absorbs losses first (high-risk, high return).  

**Workflow:**
1. Users deposit into Senior or Junior tranches.  
2. Underlying yield (from RWA) flows into the pool periodically.  
3. Smart contract allocates yield by priority.  
4. In case of loss, Junior absorbs first.  
5. Users can redeem at any time.  

---

## 🧱 Technical Highlights

- On-chain yield allocation & loss absorption logic  
- Soroban smart contract–based **risk isolation**  
- Dynamic parameters: pause, resume, rebalance  
- Automated periodic yield distribution  
- Cross-contract integration with **Blend Pool**  
- Transparent event logs for every on-chain action  

---

## 🔍 Contracts & Deployment

| Module | Contract ID / Address | Description |
|--------|----------------------|-------------|
| **Tranche Contract** | `CAIUMAVGQUDLA5EMTCC4GY5EF64VMZOFPSS6EFZZKLFWMAB56ZPE5QRP` | Core yield-tranching logic |
| **Blend Pool** | `CD24SABPPEFJHQ4D5UEVAV52SUYHDERKKBNWX2PUGVPSJ6NCOEJVBLTQ` | Simulated RWA yield source |
| **RWA Benefit Simulator** | `CCA2BWGKIB7TU5VWHZSRDSGQPSIROSHGE4RUXOW4S6RMGU4DK5EXO7BN` | Demo yield generator |
| **Admin Account** | `GAZ57ZNVBFTYPAR7EVW7LISVT5ZYU2FFHB7Q5YC74KDUXNILIVM7555Q` | Contract owner / manager |

🔗 [Tranche Contract on Stellar Expert](https://stellar.expert/explorer/testnet/contract/CAIUMAVGQUDLA5EMTCC4GY5EF64VMZOFPSS6EFZZKLFWMAB56ZPE5QRP)  
🔗 [Blend Pool Explorer Link](https://stellar.expert/explorer/testnet/contract/CD24SABPPEFJHQ4D5UEVAV52SUYHDERKKBNWX2PUGVPSJ6NCOEJVBLTQ)

---

## 🧪 Demo Highlights

✅ Senior & Junior tranche subscription / redemption  
✅ Automatic yield allocation by priority  
✅ Simulated loss handling (Junior first loss)  
✅ Pause & governance parameter updates  
✅ Full on-chain event tracking  

Example transaction:  
[Yield distribution transaction](https://stellar.expert/explorer/testnet/tx/5377406428782592)

---

## 🌍 Why It Fits Stellar

> “Brew bold ideas and experiment without limits.”

RWA Tranche embodies Stellar’s hackathon spirit:

- 🔹 **DeFi innovation** — structured yield as a native primitive  
- 🔹 **RWA liquidity activation** — making tokenized bonds productive  
- 🔹 **Smart-contract interoperability** — Soroban + Blend integration  
- 🔹 **Stablecoin-driven yield options** — risk-adjusted on-chain returns  

---

## 🧭 Vision

We aim to make **on-chain RWAs as dynamic and flexible as traditional credit markets** —  
where risk and return are transparent, programmable, and composable.

**Future directions:**
- Standardized yield-tranching module for RWA projects  
- Composable RWA-backed yield layer for DeFi  
- Institutional yield management tools built on Stellar  

---

## 🛠️ Tech Stack

- **Language:** Rust  
- **Smart Contracts:** Soroban (WASM)  
- **DeFi Protocol:** Blend Pool  
- **Network:** Stellar Testnet  
- **Tools:** Soroban CLI, Stellar SDK, Blend SDK  
- **Explorer:** [stellar.expert](https://stellar.expert)

---

## 📜 License

MIT License © 2025 **RWA Tranche Team**

---

## ✉️ Contact

Built with ❤️ by **Lindsey & Tatyana**  
For **Stellar Hackathon 2025** — *“Brew bold ideas.”*

---

