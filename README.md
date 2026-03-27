<img width="2880" height="1800" alt="image" src="https://github.com/user-attachments/assets/332bf4a1-dcf3-4125-a3fe-ff0127de8d2a" />
https://stellar.expert/explorer/testnet/contract/CCTYRFMPN5G7WCJPOYGTXIM6FNLFXTMRM4WMVIREDYYRCBXODWLNZMZS

# Skill Verification Smart Contract (Soroban)

## 📌 Project Description
This project is a decentralized **Skill Verification System** built on the Stellar Soroban smart contract platform. It allows users to register their skills and have them verified in a trustless and tamper-proof manner using blockchain technology.

## 🚀 What it does
The smart contract enables:
- Users to **add skills** to their profile
- Skills to be **verified** (by an authority or system)
- Anyone to **check verification status** of a user's skill

This can be used for:
- Resume verification
- Freelance marketplaces
- Academic or certification validation

## ✨ Features
- 🔐 **Decentralized storage** of skills
- ✅ **Verification system** for skills
- 🔍 **Publicly verifiable records**
- ⚡ Built on **Stellar Soroban** (fast & low cost)
- 🧩 Simple and extendable smart contract design

## 🛠️ Functions
- `add_skill(user, skill, verified)` → Add a skill
- `verify_skill(user, skill)` → Mark skill as verified
- `get_skill(user, skill)` → Check verification status

## 🧠 Future Improvements
- Role-based verification (only authorized verifiers)
- NFT certificates for verified skills
- Frontend dashboard (React + Stellar SDK)
- Skill endorsements (like LinkedIn)
- Multi-skill querying support

## 📦 Tech Stack
- Rust
- Soroban SDK
- Stellar Blockchain

## ⚙️ How to Run
1. Install Soroban CLI
2. Build contract:
   ```bash
   cargo build --target wasm32-unknown-unknown --release


