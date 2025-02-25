# <p align="center">  CryptoSun (CSN) </p>

<p align="center"><img src="https://yellow-negative-parrotfish-381.mypinata.cloud/ipfs/bafybeibpaqueerbadvpiamxqczpqbauxiteebdcrt2yakp3ul7dxtw4nr4" alt="CryptoSun Logo" width="300"></p>


# Overview
A (DePIN) Token on Solana integrating solar energy, heating, and blockchain utility mixed with real-world energy utility.
CryptoSun (CSN) is a utility token developed by Absolute Solar & Crypto Inc. Dev team(Currently:Nickalos Gonzales,), built on the Solana blockchain to tokenize solar energy for heating, bitcoin mining, and peer-to-peer economic empowerment. Leveraging Solana’s high-throughput, CSN powers a sustainable ecosystem with staking rewards, quarterly dividends, and governance rights. The project aims to redefine renewable energy infrastructure through blockchain, delivering real-world utility and financial incentives.

# Roadmap

    Q3 2025: Core platform & token launch Full integration in Website Buying, Selling, Transfer Go Live on DEX!
    Q3 2025: Staking and Burning Implementaion
    Q4 2026: Try to get listed on a CEX
    Q4 2025: Governance Features Implemented for Staking & Burning
    Q4 2025: Governance Integration with website
    Q1 2026: Dividend Features Implementation
    Q3 2026: Automated Mainteance & Rewards Features 
    Q1 2027: Energy Trading Features
    Q3 2027: Govenance Features Where needed
    Q1 2028: DeFi expansion
    Q1 2028: Cross-chain bridges
    Q3 2028: AI-driven staking
    2028-2030: Global scale (1M panels, 10M users)(USA Operations)

# Current Tokenomics
    
    Token: CSN
    Initial Supply: 100,000,000 CSN
    Circulating at Launch: 19,600,000 CSN
    Initial Market Cap: $20,000 ($0.001/CSN)
    Utility: Staking, Energy Trading, Governance, Maintenance/Upkeep, Real-World Solar & Heating Applications


# Features

    Solar Energy Integration: Converts excess solar power into CSN via ASIC mining, with heat repurposed for furnaces. (Future Development)
    Economic Model: Hybrid model. Inflationary (Years 1-2) to deflationary (Year 3+), with burns reducing supply to 92.9M CSN by Year 6. (Completed)
    Staking Rewards: 42M CSN allocated, 20% quarterly dividends for top stakers from Absolute Solar & Crypto Inc. (Completed)
    Smart Contracts: Rust-based, Managing token operations, Staking, Governance, and more. (Future Development)
    Security: Ed25519 cryptography, zero-knowledge proofs, continuous audits, and bug bounties. (Future Development)
    Roadmap: Token Contract, Staking Contract, Governance Contract, Burn Contract, Dividend Distribution, Maintenance Contract, Energy Trading!, Full Application integration.(Future Development)


# Technical Details

    Blockchain: Solana
    Throughput: ~65,000 TPS (peak 700,000 TPS)
    Cryptography: Ed25519 (128-bit security)
    Consensus: Proof of History (PoH) + Proof of Stake (PoS) + Tower BFT
    Programming: Rust, compiled to BPF bytecode
    Serialization: Borsh (150-200 byte tx size)
    Fees: Base 0.000005 SOL/signature, prioritization adjustable

# Smart Contracts

    Token: Defines CSN (100M supply, 9 decimals, 1M CSN/year minting).
    Staking: Locks 42M CSN, rewards via Reward = 0.0001 · Stake · (EnergyFactor + UptimeFactor + MaintenanceFactor).
    Governance: 4M CSN, vested Months 13-24, 2/3 vote consensus.
    Burn: Reduces supply (e.g., 3.5M CSN/year by Year 6).
    Dividend: Airdrops based on Dividends through Company revenue.
    Future: Maintenance (IoT-driven), Energy trading (LMP pricing).

<br>
<br>
<br>

# Installation(Windows)
Ref: <a>https://learn.microsoft.com/en-us/windows/wsl/install<a>

    bash
    wsl --install
    wsl
Now follow the rest of the Linux installation

# Installation(Linux) 
Download Dependencies:

    Rust: cargo --version >= 1.70
    Solana CLI: solana --version >= 1.18
    Node.js: node --version >= 16 (for tools)
    Git: git --version

Quick Install <a>https://solana.com/docs/intro/installation</a>

    curl --proto '=https' --tlsv1.2 -sSfL https://raw.githubusercontent.com/solana-developers/solana-install/main/install.sh | bash
    

# Setup(Bash)
<p>Clone Repo</p>
<p>Install dependencies</p>
<p>Set up Next.js</p>
<p>Set up Node</p>
<p>Configure Solana</p>
<p>Deploy Contracts</p>

<h2>Clone the Repository:</h2>  
git clone <a>https://github.com/rednickk1/CryptoSun.git</a>

    cd CrytpoSun

<h2>Install Dependencies:</h2>
    
    cargo build --release
    npm install

<h2>Setting up Next.js Environment:</h2>

    npx create-solana-dapp

<h2>Configure Next.js:</h2>

    Project name:
    Preset:
    UI Lib:
    Anchor Template:
    cd /your-project-directory

<h2>Web Front End:</h2>

    npm i
    npm run dev
<a>http://localhost:3000</a>

<h2>Setting up Solana Validator:</h2>

install phantom wallet
<a>https://phantom.com/</a>
Change to devnet or testnet

    solana-test-validator

<h2>Configure Solana:</h2>
    
    solana config set --url https://api.devnet-beta.solana.com
    solana-keygen new

<h2>Deploy Contracts:</h2>

    solana program deploy target/deploy/csn_token.so

<h2>Usage:</h2>

    Compile: cargo build 
    Test: cargo test
    Interact: Use Solana CLI or SDK (e.g., @solana/web3.js) to call contracts.
        Example: Transfer CSN
        javascript

        const { PublicKey, Transaction } = require('@solana/web3.js');
        // Add transfer logic here

# Contributions
Contributing (Currently only Absolute Solars Dev Team possible open-source after Launch)
Open-source Devs! Please follow these steps:
//removed bullet points here//
Fork the repository and create a feature branch with your changes. Ensure code adheres to Rust best practices and Solana’s security standards. Submit a pull request with a clear description of your contribution. Issues can be reported via GitHub Issues—focus on bugs, feature requests, or security enhancements.
Security

    Audits: Conducted by CyberScope and CertiK or reputable sources.
    Bounties: Report vulnerabilities to earn $1,000-$50,000 (CSN).
    Contact: devnickk@proton.me

# License
This project is licensed under the MIT License. See LICENSE for details.
Resources

    Website: CryptoSun.ca
    Whitepaper: CSN Whitepaper
    Solana Docs: docs.solana.com
    Contact: devnickk@proton.me
    @Absolute Solar & Crypto inc.
