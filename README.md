# CryptoSun (CSN)
![CryptoSun Logo](https://yellow-negative-parrotfish-381.mypinata.cloud/ipfs/bafybeibpaqueerbadvpiamxqczpqbauxiteebdcrt2yakp3ul7dxtw4nr4)

A (DePIN) Token on Solana integrating solar energy, heating, and blockchain utility mixed with real-world energy utility.
Overview
CryptoSun (CSN) is a utility token developed by Absolute Solar & Crypto Inc. Dev team(Currently:Nickalos Gonzales,), built on the Solana blockchain to tokenize solar energy for heating, bitcoin mining, and peer-to-peer economic empowerment. Leveraging Solana’s high-throughput, CSN powers a sustainable ecosystem with staking rewards, quarterly dividends, and governance rights. The project aims to redefine renewable energy infrastructure through blockchain, delivering real-world utility and financial incentives.

Roadmap

    Q3 2025: Core platform & token launch Full integration in Website.
    Q3 2025: Buying, Selling, Transfer Go Live on DEX!
    Q3 2025: Staking and Burning Implementaion 
    Q4 2025: Governance Features Implemented for Staking & Burning
    Q4 2025: Governance Integration with website
    Q4 2026: Dividend Features Implementation
    Q1 2026: Automated Mainteance & Rewards Features 
    Q2 2026: Energy Trading Features
    Q2 2026: Try to get listed on a CEX
    Q3 2026: Govenance Features Where needed
    Q4 2026: DeFi expansion
    Q1 2027: Cross-chain bridges
    Q2 2027: AI-driven staking
    2028-2030: Global scale (1M panels, 10M users)(USA Operations)

Current Tokenomics
    
    Token: CSN
    Initial Supply: 100,000,000 CSN
    Circulating at Launch: 19,600,000 CSN
    Initial Market Cap: $20,000 ($0.001/CSN)
    Utility: Staking, Energy Trading, Governance, Maintenance/Upkeep, Real-World Solar & Heating Applications


Features

    Solar Energy Integration: Converts excess solar power into CSN via ASIC mining, with heat repurposed for furnaces. (Future Development)
    Economic Model: Hybrid model. Inflationary (Years 1-2) to deflationary (Year 3+), with burns reducing supply to 92.9M CSN by Year 6. (Completed)
    Staking Rewards: 42M CSN allocated, 20% quarterly dividends for top stakers from Absolute Solar & Crypto Inc. (Completed)
    Smart Contracts: Rust-based, Managing token operations, Staking, Governance, and more. (Future Development)
    Security: Ed25519 cryptography, zero-knowledge proofs, continuous audits, and bug bounties. (Future Development)
    Roadmap: Token Contract, Staking Contract, Governance Contract, Burn Contract, Dividend Distribution, Maintenance Contract, Energy Trading!, Full Application integration.(Future Development)


Technical Details

    Blockchain: Solana
    Throughput: ~65,000 TPS (peak 700,000 TPS)
    Cryptography: Ed25519 (128-bit security)
    Consensus: Proof of History (PoH) + Proof of Stake (PoS) + Tower BFT
    Programming: Rust, compiled to BPF bytecode
    Serialization: Borsh (150-200 byte tx size)
    Fees: Base 0.000005 SOL/signature, prioritization adjustable

Smart Contracts

    Token: Defines CSN (100M supply, 9 decimals, 1M CSN/year minting).
    Staking: Locks 42M CSN, rewards via Reward = 0.0001 · Stake · (EnergyFactor + UptimeFactor + MaintenanceFactor).
    Governance: 4M CSN, vested Months 13-24, 2/3 vote consensus.
    Burn: Reduces supply (e.g., 3.5M CSN/year by Year 6).
    Dividend: Airdrops based on Dividends through Company revenue.
    Future: Maintenance (IoT-driven), Energy trading (LMP pricing).


# Installation
Download Dependencies

    Rust: cargo --version >= 1.70
    Solana CLI: solana --version >= 1.18
    Node.js: node --version >= 16 (for tools)
    Git: git --version

Setup

Clone the Repository:
    
    bash
    git clone https://github.com/AbsoluteSolarCrypto/CSN.git
    cd CSN

Install Dependencies:
    
    bash
    cargo build --release
    npm install

Configure Solana:
    
    bash
    solana config set --url https://api.mainnet-beta.solana.com
    solana-keygen new

Deploy Contracts:

    bash
    solana program deploy target/deploy/csn_token.so

Usage:

    Compile: cargo build --release
    Test: cargo test
    Interact: Use Solana CLI or SDK (e.g., @solana/web3.js) to call contracts.
        Example: Transfer CSN
        javascript

        const { PublicKey, Transaction } = require('@solana/web3.js');
        // Add transfer logic here


Contributing (Currently only Absolute Solars Dev Team possible open-source after Launch)
Open-source Devs! Please follow these steps:
//removed bullet points here//
Fork the repository and create a feature branch with your changes. Ensure code adheres to Rust best practices and Solana’s security standards. Submit a pull request with a clear description of your contribution. Issues can be reported via GitHub Issues—focus on bugs, feature requests, or security enhancements.
Security

    Audits: Conducted by CyberScope and CertiK or reputable sources.
    Bounties: Report vulnerabilities to earn $1,000-$50,000 (CSN).
    Contact: devnickk@proton.me

License
This project is licensed under the MIT License. See LICENSE for details.
Resources

    Website: CryptoSun.ca
    Whitepaper: CSN Whitepaper
    Solana Docs: docs.solana.com
    Contact: devnickk@proton.me
    @Absolute Solar & Crypto inc.
