# 🟢 STK Staking — Green Belt

[![CI](https://github.com/Nikkunj-145/stk-staking/actions/workflows/ci.yml/badge.svg)](https://github.com/Nikkunj-145/stk-staking/actions/workflows/ci.yml)
[![Network](https://img.shields.io/badge/network-Stellar%20Testnet-14b8a6)](https://stellar.expert/explorer/testnet)
[![Tests](https://img.shields.io/badge/tests-15%20passing-22c55e)](#-tests)
[![License](https://img.shields.io/badge/license-MIT-blue)](#-license)

A two-contract DeFi mini-dApp on Stellar Soroban demonstrating **inter-contract calls**, an open token **faucet**, real-time on-chain data, mobile-responsive UI, and a CI/CD pipeline.

> **Stellar Frontend Challenge — Level 4 (Green Belt) submission.**

**🌐 Live demo:** _Deploying to Render — URL will be added here once live._

---

## 🎯 What it does

Users connect a wallet, **stake** an SEP-41-style fungible token (`STK`) into a pool, earn **points** that accrue linearly per token-second, and **claim** or **unstake** any time. The staking contract performs **inter-contract calls** to the token contract's `transfer` function to actually move tokens in and out of the pool.

---

## 🚀 Live deployment (testnet)

| Contract        | Address | Explorer |
| --------------- | ------- | -------- |
| **Token (STK)** | `CDUECTBOEY4HIGBKXNRW25LB4SYVI4UAUDME2LYAE6T6K5RRFTWCIEDY` | [view](https://stellar.expert/explorer/testnet/contract/CDUECTBOEY4HIGBKXNRW25LB4SYVI4UAUDME2LYAE6T6K5RRFTWCIEDY) |
| **Staking pool**| `CBT4AXYW2RL6QG2QA2C7P4UOBEZRVJLGQW6764NTB6WZBFJNM2SWXD2R` | [view](https://stellar.expert/explorer/testnet/contract/CBT4AXYW2RL6QG2QA2C7P4UOBEZRVJLGQW6764NTB6WZBFJNM2SWXD2R) |

### Sample on-chain transactions

| Action | Tx hash |
| ------ | ------- |
| Token deploy | [`f19af11e60433a162f4f93cf7263ee98048e2dce9e28276c1bb4281a7afe1cbd`](https://stellar.expert/explorer/testnet/tx/f19af11e60433a162f4f93cf7263ee98048e2dce9e28276c1bb4281a7afe1cbd) |
| Staking deploy | [`ce158073be95dd35f78c0ae0ad4bdb85bdda7e7df7332a1643682a4bff8f1df8`](https://stellar.expert/explorer/testnet/tx/ce158073be95dd35f78c0ae0ad4bdb85bdda7e7df7332a1643682a4bff8f1df8) |
| Faucet claim | [`ccb5d7c0add2d17cf32063e37200a3c20e24ed93ecc771b031cd5139abe9b4cf`](https://stellar.expert/explorer/testnet/tx/ccb5d7c0add2d17cf32063e37200a3c20e24ed93ecc771b031cd5139abe9b4cf) |

Reward formula: `points = staked_amount × seconds_elapsed × 100 / 1e9`

---

## ✨ Features

- 🔗 **Inter-contract calls** — staking pool invokes token's `transfer` & `balance`
- 💧 **Open faucet** — any user can claim 1,000 STK once to try the dApp (no admin needed)
- 🪪 **Multi-wallet** via StellarWalletsKit (Freighter, xBull, Albedo, Lobstr, Hana)
- 🔄 **Switch-account button** — fresh wallet picker on every connect
- 📱 **Mobile responsive** — works edge-to-edge on phones
- ⏱️ **Live points ticker** — auto-refresh every 4 s while connected
- 🎚️ Stake / Unstake / Claim with **Max** quick-fill
- 🧮 Bigint-safe amount math (7-decimal token)
- 📊 4 live stats: wallet · staked · pending points · pool TVL
- 🛡️ **6 contract error variants** mapped to friendly UI:
  - `NotInitialized`, `AlreadyInitialized`, `InsufficientBalance`, `NegativeAmount`, `AlreadyClaimed`, `NothingStaked`
- ✅ **15 unit tests** (7 token + 8 staking) all passing
- 🤖 **GitHub Actions CI** — runs `cargo test --workspace` + `npm run build` on every push
- 🛟 **ErrorBoundary** — catches render-time crashes and displays a graceful fallback

---

## 🧪 Tests

```bash
cargo test --workspace
```

```
running 7 tests (token)
test test::init_sets_metadata           ok
test test::mint_and_balance             ok
test test::transfer_moves_balance       ok
test test::transfer_insufficient_fails  ok
test test::negative_amount_rejected     ok
test test::faucet_grants_tokens_once    ok
test test::faucet_double_claim_rejected ok
test result: ok. 7 passed

running 8 tests (staking)
test test::stake_locks_tokens_into_pool      ok
test test::unstake_returns_tokens            ok
test test::rewards_accrue_over_time          ok
test test::double_init_rejected              ok
test test::unstake_without_stake_fails       ok
test test::cannot_unstake_more_than_staked   ok
test test::negative_amount_rejected          ok
test test::claim_settles_pending_into_points ok
test result: ok. 8 passed
```

---

## 🧰 Tech stack

| Layer       | Tech |
| ----------- | ---- |
| Contracts   | Rust + `soroban-sdk` 22 (workspace, 2 members) |
| Frontend    | React 18 + Vite + TypeScript |
| Styling     | TailwindCSS (mobile-first) + Lucide |
| Wallets     | StellarWalletsKit |
| Stellar SDK | `@stellar/stellar-sdk` 14 (Protocol 23) |
| CI          | GitHub Actions (Rust + Node matrix) |

---

## 📁 Structure

```
green-belt/
├── contracts/
│   ├── token/           # SEP-41-style fungible token (5 tests)
│   └── staking/         # Pool with inter-contract calls (8 tests)
├── src/
│   ├── components/      # WalletBar, Stats, StakePanel
│   ├── lib/             # config, wallet, stellar (RPC), format
│   ├── App.tsx
│   ├── main.tsx
│   └── index.css
├── .github/workflows/ci.yml   # CI: cargo test + frontend build
├── render.yaml                 # Render Blueprint (one-click deploy)
├── netlify.toml                # Netlify build config (alt deploy)
├── deployment.json
├── .env.example
└── README.md
```

---

## 🛠️ Setup

### Prerequisites
- Node 18+
- Rust + `wasm32v1-none` target
- Stellar CLI ≥22
- Wallet extension (Freighter recommended) on **testnet**

### Run
```bash
git clone https://github.com/Nikkunj-145/stk-staking.git
cd stk-staking
npm install
cp .env.example .env.local   # already pre-filled with deployed contracts
npm run dev
```
Open http://localhost:5176

### Re-deploy
```bash
stellar contract build
stellar contract deploy --wasm target/wasm32v1-none/release/token.wasm --source <id> --network testnet
stellar contract deploy --wasm target/wasm32v1-none/release/staking.wasm --source <id> --network testnet
# Init token (admin, decimals, name, symbol):
stellar contract invoke --id <TOKEN_ID> --source <id> --network testnet -- init \
  --admin <YOUR_ADDR> --decimal 7 --name '"Stellar Stake Token"' --symbol '"STK"'
# Init staking with token addr:
stellar contract invoke --id <STAKING_ID> --source <id> --network testnet -- init \
  --token <TOKEN_ID>
# Mint test tokens to a user:
stellar contract invoke --id <TOKEN_ID> --source <id> --network testnet -- mint \
  --to <USER_ADDR> --amount 10000000
```

> **Tip**: New users can claim **1,000 STK from the in-app faucet** (one-time per wallet). Admin `mint` is also available via CLI for arbitrary amounts.

---

## 🚀 Deploy your own (Render)

The repo ships a `render.yaml` Blueprint, so you can deploy a copy in two clicks:

1. Fork this repo on GitHub.
2. Open https://dashboard.render.com → **New +** → **Blueprint** → connect your fork.
3. Render reads `render.yaml`, runs `npm ci && npm run build`, and serves `dist/` as a static site with all `VITE_*` env vars pre-filled.
4. SPA fallback (`/* → /index.html`) and asset caching headers are configured automatically.

---

## 🧠 Architecture: inter-contract calls

The staking contract defines a minimal `TokenInterface` trait via `#[contractclient]` so the Soroban host can invoke any SEP-41-style token from inside `stake()` / `unstake()`:

```rust
#[contractclient(name = "TokenClient")]
pub trait TokenInterface {
    fn transfer(env: Env, from: Address, to: Address, amount: i128);
    fn balance(env: Env, id: Address) -> i128;
}

// Inside `stake`:
let token_client = TokenClient::new(&env, &token_addr);
token_client.transfer(&user, &pool, &amount);  // <-- inter-contract call
```

The user signs a single tx that authorizes both:
1. The outer `staking::stake` call.
2. The inner `token::transfer(user → pool)` invoked sub-call.

Soroban's auth framework propagates the `require_auth()` automatically through the call stack.

---

## 📸 Screenshots

### CI/CD pipeline

![CI status](https://github.com/Nikkunj-145/stk-staking/actions/workflows/ci.yml/badge.svg)

The CI badge above is live — it goes green when both jobs (`Build + Test contracts` and `Build frontend`) succeed on `main`.

### Mobile responsive

_Screenshot to be added: open the live demo on a phone (or DevTools → mobile mode 375 × 667) and capture the staking page._

---

## ✅ Submission checklist (Green Belt)

- [x] **2 deployed contracts** demonstrating inter-contract calls
- [x] **15 unit tests passing**
- [x] **6 contract error variants** mapped to UI
- [x] Multi-wallet support + switch-account button
- [x] **Open faucet** (custom token mechanic)
- [x] **Mobile-responsive UI** (Tailwind mobile-first, edge-to-edge)
- [x] **Live data ticker** (4 s polling)
- [x] Comprehensive README with badges, contract addresses, and tx hashes
- [x] **GitHub Actions CI** (cargo test + frontend build + dist artifact)
- [x] **15+ meaningful commits** in repo history
- [x] **Render Blueprint** (`render.yaml`) for one-click deploy
- [ ] Live deploy URL (paste here once Render finishes build)

---

## 📜 License

MIT © Nikkunj
