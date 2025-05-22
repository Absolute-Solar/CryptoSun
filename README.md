# CryptoSun Program Library (CPL)

[![Latest Release](https://img.shields.io/github/v/release/Absolute-Solar/CryptoSun-Program-Library)](https://github.com/Absolute-Solar/CryptoSun-Program-Library/releases)
[![docs](https://img.shields.io/badge/docs-V1.0.0-darkblue)](https://github.com/Absolute-Solar/Docs)
[![Website](https://img.shields.io/badge/website-visit-green)](https://3rdtest.webflow.io/)

The CryptoSun Program Library (CPL) is the core collection of Solana programs that power the CryptoSun renewable energy network. This repository contains the fundamental smart contracts that enable tokenized energy, DePIN infrastructure, and sustainable crypto mining on the Solana blockchain. CryptoSun is a revolutionary renewable energy network bringing programmable energy to the Solana blockchain. Our Decentralized Physical Infrastructure Network (DePIN) transforms tokenized energy into a thriving token ecosystem, creating a sustainable bridge between renewable resources and blockchain technology.

### Vision & Mission

CryptoSun creates a self-sustaining ecosystem where solar energy directly powers blockchain infrastructure while repurposing wastede heat, establishing a complete energy lifecycle on Solana. By tokenizing renewable energy and providing incentives for sustainable mining/staking, we're building a greener future for blockchain technology. Users are able to benefit from various rewards systems by using the Physical Infrastructure which incentizes users to interact with the system to earn and lower heating bills.

### Core Components

The CryptoSun DePIN ecosystem consists of several integrated components:

1. **Energy Tokenization**: Convertible tokens representing solar energy units
2. **Mining Operations**: Solar-powered crypto mining hardware
3. **Heat Recovery**: Repurposing waste heat from mining operations
4. **Reward Distribution**: Incentivizing network participation and growth
5. **Governance**: Community-led decision making

## Architecture

The CryptoSun protocol is built as a collection of interdependent Solana programs:

```
┌─────────────────────────────┐     ┌─────────────────────────────┐
│                             │     │                             │
│      CSN Token Program      │◄────┤    Energy Oracle Program    │
│                             │     │                             │
└───────────┬─────────────────┘     └─────────────┬───────────────┘
            │                                     │
            │                                     │
            ▼                                     ▼
┌─────────────────────────────┐     ┌─────────────────────────────┐
│                             │     │                             │
│      Staking Program        │     │    Mining Registry Program  │
│                             │     │                             │
└───────────┬─────────────────┘     └─────────────┬───────────────┘
            │                                     │
            │                                     │
            ▼                                     ▼
┌─────────────────────────────┐     ┌─────────────────────────────┐
│                             │     │                             │
│      Governance Program     │◄────┤    Energy Market Program    │
│                             │     │                             │
└─────────────────────────────┘     └─────────────────────────────┘
```

### Data Flow

1. **Energy Production**: Solar hardware produces energy and reports to the Energy Oracle
2. **Tokenization**: The Oracle verifies energy data and calculates CSN tokens rewards
3. **Mining**: Tokens power crypto mining operations
4. **Heat Utilization**: Waste heat is captured and repurposed
5. **Rewards**: Participants earn rewards based on energy contribution
6. **Governance**: Token holders vote on protocol improvements

## Programs

The CryptoSun Program Library contains the following Solana programs:

| Program | Description | Program ID |
|---------|-------------|------------|
| CSN Token | Core token implementation with energy attributes | `csn1...` |
| Energy Oracle | Connects real-world energy data to the blockchain | `eorg...` |
| Staking | CSN token staking and rewards distribution | `stak...` |
| Mining Registry | Registers and tracks mining hardware | `minr...` |
| Energy Market | P2P marketplace for tokenized energy | `emkt...` |
| Governance | On-chain voting and proposal system | `govn...` |
| Heat Recovery | Tracks and rewards waste heat utilization | `heat...` |

## Getting Started

### Prerequisites

- [Solana Tool Suite](https://docs.solana.com/cli/install-solana-cli-tools) v1.14.0 or later
- [Anchor](https://project-serum.github.io/anchor/getting-started/installation.html) v0.28.0 or later
- [Node.js](https://nodejs.org/) v16 or later
- [Yarn](https://yarnpkg.com/) v3 or later
- [Rust](https://www.rust-lang.org/tools/install) v1.68.0 or later

### Installation

```bash
# Clone the repository
git clone https://github.com/cryptosun/cryptosun-program-library.git
cd cryptosun-program-library

# Install dependencies
yarn install

# Build all programs
yarn build
```

### Local Development

```bash
# Start a local Solana validator
solana-test-validator

# Deploy programs to localnet
anchor deploy

# Run integration tests
yarn test
```

## Program Interactions

### Token Program

The CSN Token program implements the Solana SPL Token standard with additional energy-specific features:

```rust
// Example: Mint tokens based on energy production
pub fn mint_from_energy(
    ctx: Context<MintFromEnergy>,
    energy_amount: u64,
    timestamp: i64,
    signature: [u8; 64],
) -> Result<()> {
    // Verify energy oracle signature
    verify_oracle_signature(&ctx, energy_amount, timestamp, &signature)?;
    
    // Calculate token amount from energy
    let token_amount = calculate_token_from_energy(energy_amount);
    
    // Mint tokens to the producer
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.producer_token_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            &[&[MINT_AUTHORITY_SEED, &[ctx.bumps.mint_authority]]],
        ),
        token_amount,
    )?;
    
    Ok(())
}
```

### Energy Oracle Program

The Energy Oracle program validates and records energy production from registered solar hardware:

```rust
// Example: Submit energy production data
pub fn submit_energy_production(
    ctx: Context<SubmitEnergyProduction>,
    amount: u64,
    timestamp: i64,
    hardware_signature: [u8; 64],
) -> Result<()> {
    // Verify hardware is registered and signature is valid
    verify_hardware_signature(
        &ctx.accounts.hardware_registry,
        &ctx.accounts.producer, 
        amount, 
        timestamp, 
        &hardware_signature
    )?;
    
    // Record energy production
    let production = &mut ctx.accounts.energy_production;
    production.amount = amount;
    production.timestamp = timestamp;
    production.producer = ctx.accounts.producer.key();
    production.verified = true;
    
    // Generate oracle signature for token minting
    let oracle_signature = generate_oracle_signature(amount, timestamp)?;
    production.oracle_signature = oracle_signature;
    
    Ok(())
}
```

## Absolute Solar DePIN Packages

CryptoSun offers hardware packages for participants to join the network:

| Package | Description | Energy Production | Reward Potential |
|---------|-------------|-------------------|------------------|
| Solar Miner Basic | 1kW solar + entry mining rig | ~4 kWh/day | ~10 CSN/day |
| Solar Miner Pro | 3kW solar + mid-tier mining | ~12 kWh/day | ~35 CSN/day |
| Solar Farm Node | 10kW+ commercial setup | ~40+ kWh/day | ~120+ CSN/day |
| Heat Recovery Add-on | Waste heat utilization | N/A | +20% bonus |

## Deployments

### Mainnet

| Program | Address | Version |
|---------|---------|---------|
| CSN Token | `csn1...` | v1.0.0 |
| Energy Oracle | `eorg...` | v1.0.0 |
| Staking | `stak...` | v1.0.0 |
| Mining Registry | `minr...` | v1.0.0 |
| Energy Market | `emkt...` | v1.0.0 |

### Devnet

| Program | Address | Version |
|---------|---------|---------|
| CSN Token | `devcsn1...` | v1.0.0 |
| Energy Oracle | `deveorg...` | v1.0.0 |
| Staking | `devstak...` | v1.0.0 |
| Mining Registry | `devminr...` | v1.0.0 |
| Energy Market | `devemkt...` | v1.0.0 |

## Security

### Audit Status

| Program | Audit Firm | Status | Report |
|---------|------------|--------|--------|
| CSN Token | [Audit Firm] | Completed | [Link to report] |
| Energy Oracle | [Audit Firm] | In Progress | ETA: Q2 2025 |
| Staking | [Audit Firm] | Completed | [Link to report] |
| Others | - | Planned | Q3 2025 |

### Security Features

- **Hardware Attestation**: Cryptographic verification of energy production hardware
- **Multi-signature Control**: Critical protocol parameters protected by multi-sig
- **Oracle Fault Tolerance**: Energy data validation through multiple data sources
- **Rate Limiting**: Protection against flash attacks and manipulation

## Development

### Repository Structure

```
cryptosun-program-library/
├── programs/
│   ├── csn-token/
│   ├── energy-oracle/
│   ├── staking/
│   ├── mining-registry/
│   ├── energy-market/
│   ├── governance/
│   └── heat-recovery/
├── tests/
│   ├── token-tests/
│   ├── oracle-tests/
│   ├── staking-tests/
│   ├── integration-tests/
│   └── security-tests/
├── clients/
│   ├── js/
│   └── rust/
├── scripts/
│   ├── deployment/
│   ├── migration/
│   └── utilities/
├── docs/
│   ├── architecture/
│   ├── api/
│   └── examples/
└── packages/
    ├── common/
    └── types/
```

### Development Workflow

1. **Feature Branches**: Create branches for new features or fixes
2. **Testing**: All changes require tests with adequate coverage
3. **Code Review**: At least two approvals needed for merges
4. **CI/CD**: Automated testing and deployment pipeline
5. **Documentation**: API docs generated and examples updated
6. **Security**: Vulnerability scanning before release

## Smart Contract Upgradeability

CryptoSun programs implement the following upgradeability patterns:

| Program | Upgrade Method | Authority |
|---------|---------------|-----------|
| CSN Token | Upgradable BPF | Governance multi-sig |
| Energy Oracle | Upgradable BPF | Governance multi-sig |
| Staking | Proxy pattern | Governance multi-sig |
| Others | Upgradable BPF | Governance multi-sig |

## Energy Sustainability

CryptoSun's commitment to sustainability is embedded in the protocol:

- **Energy Efficiency Rewards**: Bonus rewards for more efficient mining setups
- **Carbon Tracking**: On-chain tracking of carbon offset from solar usage
- **Hardware Recycling**: Incentives for proper recycling of outdated hardware
- **Community Sustainability Fund**: Portion of fees dedicated to environmental initiatives

## Integration & SDKs

### JavaScript/TypeScript SDK

```typescript
import { Connection } from '@solana/web3.js';
import { CryptoSun } from '@cryptosun/sdk';

// Initialize client
const connection = new Connection('https://api.mainnet-beta.solana.com');
const cryptosun = new CryptoSun(connection);

// Register a new solar miner
await cryptosun.registerMiner({
  hardwareId: 'SM-1000-XYZ',
  capacity: 3000, // 3kW
  location: { lat: 37.7749, lng: -122.4194 },
});

// Submit energy production
await cryptosun.submitEnergyProduction({
  amount: 15000, // 15 kWh
  timestamp: Date.now(),
  hardwareSignature: '...',
});
```

### Rust Client

```rust
use cryptosun_client::{CryptoSunClient, SolarMiner, EnergyProduction};
use solana_client::rpc_client::RpcClient;

// Initialize client
let rpc = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
let client = CryptoSunClient::new(rpc);

// Register a new solar miner
let miner = SolarMiner {
    hardware_id: "SM-1000-XYZ".to_string(),
    capacity: 3000, // 3kW
    location: Location { lat: 37.7749, lng: -122.4194 },
};
client.register_miner(miner)?;

// Submit energy production
let production = EnergyProduction {
    amount: 15000, // 15 kWh
    timestamp: chrono::Utc::now().timestamp(),
    hardware_signature: [/* ... */],
};
client.submit_energy_production(production)?;
```

## Roadmap

### Current Phase (Q2 2025)
- [x] Core token and energy oracle implementation
- [x] Basic staking functionality
- [x] Mining registry
- [ ] Energy marketplace MVP

### Next Phase (Q3-Q4 2025)
- [ ] Enhanced heat recovery tracking
- [ ] Cross-chain energy bridges
- [ ] Advanced governance features
- [ ] Regional energy marketplaces

### Future Development (2026+)
- [ ] Additional renewable sources (wind, hydro)
- [ ] Energy derivatives and futures
- [ ] Carbon credit integration
- [ ] AI-optimized energy distribution

## Community & Ecosystem

- [Website](https://cryptosun.io)
- [Discord](https://discord.gg/cryptosun)
- [Twitter](https://twitter.com/cryptosun)
- [Telegram](https://t.me/cryptosun)
- [Developer Forum](https://forum.cryptosun.io)

## Contributing

We welcome contributions to the CryptoSun Program Library! Please check our [Contributing Guidelines](CONTRIBUTING.md) before submitting pull requests.

### Contribution Process

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Run tests locally (`yarn test`)
4. Commit your changes (`git commit -m 'Add amazing feature'`)
5. Push to the branch (`git push origin feature/amazing-feature`)
6. Open a Pull Request

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.
