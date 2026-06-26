# Liamm

### One-line Description
Decentralized supply chain escrow ensuring instant agricultural factoring and payouts for smallholder rice farmers in Southeast Asia.

### Problem
An independent rice farmer in Central Luzon, Philippines, faces a 30-day delay and a 15% predatory financing fee from local middlemen to get paid for their harvest, leading to severe seasonal cash flow shortages that prevent them from purchasing seeds for the next crop cycle.

### Solution
The farmer scans a digital delivery receipt on-chain to instantly secure a micro-advance from a local cooperative pool using a Soroban smart contract, which locks the delivery state and automatically settles the payout in digital stablecoins instantly via Stellar’s low-cost ledger.

### Timeline
Developed and compiled during the 2026 Stellar Bootcamp.

### Stellar Features Used
- Soroban Smart Contracts
- Custom Fiat Stablecoin Transfers
- Persistent Contract Storage Layouts

### Vision and Purpose
To break reliance on predatory supply chain intermediaries, establishing a decentralized tracking framework that empowers unbanked agricultural workers globally to achieve self-sovereign financial security.

---

## Prerequisites
- [Rust Toolchain](https://rustup.rs/) (v1.75+)
- Target: `wasm32-unknown-unknown`
- [Stellar CLI](https://developers.stellar.org/docs/smart-contracts/getting-started/setup)

## Build Instructions
Compile the project to optimized WebAssembly targets:
```bash
soroban contract build
## Deployed Contract

| Field | Value |
|-------|-------|
| Contract ID | `CAHKCOSFCPDH6WTDEXXHMLENTEPNYO2THP74ASQ5CEQJZ7WVUBUQT4BQ` |
| Network | testnet |
| Explorer | [View on stellar.expert](https://stellar.expert/explorer/testnet/contract/CAHKCOSFCPDH6WTDEXXHMLENTEPNYO2THP74ASQ5CEQJZ7WVUBUQT4BQ) |
| Deploy Tx | [View transaction](https://stellar.expert/explorer/testnet/tx/6782569fbe8acaa31a6d7387f31ae141dc5796c4e427b8432829887e6e8e19a3) |
| Deployed | 2026-06-26 08:11:45 UTC |
| Wallet | freighter (`GDS7…BBTK`) |
![Screenshot 2026-06-26 161204.png](./assets/Screenshot%202026-06-26%20161204.png)