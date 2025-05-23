# CryptoSun Program Library

[![Latest Release](https://img.shields.io/github/v/release/Absolute-Solar/CryptoSun-Program-Library)](https://github.com/Absolute-Solar/CryptoSun-Program-Library/releases)
[![docs](https://img.shields.io/badge/docs-V1.0.0-darkblue)](https://github.com/Absolute-Solar/Docs)
[![Website](https://img.shields.io/badge/website-visit-darkgreen)](https://3rdtest.webflow.io/)
[![Discord](https://img.shields.io/badge/Join%20Server-7289da?style=flat&logo=discord&logoColor=white)](https://discord.gg/2kAqZHr3yW)
[![X](https://img.shields.io/badge/@CryptoSun_CSN-000000?style=flat&logo=x&logoColor=white)](https://twitter.com/CryptoSun_CSN)

The CryptoSun Program Library is the core collection of Solana programs that power the CryptoSun renewable energy network. These core programs create a strong foundation of code for renewable transactions on the solana network. This unleashes the untapped power of solar energy into the crypto space. Repurposed heat now becomes a lifeline of the digital crypto ecosystem backing CryptoSun with real-world energy. User benefit through various rewards for being renewable.

## Core Components

The CryptoSun DePIN ecosystem consists of several integrated components:

1. **Energy Tokenization**: Convertible tokens representing solar energy units
2. **Mining Operations**: Solar-powered crypto mining hardware
3. **Heat Recovery**: Repurposing waste heat from mining operations
4. **Reward Distribution**: Incentivizing network participation and growth

## Documentation
For the extensive detailed documentation, visit our docs page <a href="https://3rdtest.webflow.io/docs/getting-started">ABS Docs</a>

### Overview

The CryptoSun protocol is built as a collection of interdependent Solana programs. These contracts work in tandem to produce a Solana token and a DePIN infrastucture. Interconnection between Applications, Smart Contracts, and Core Ecosystem componets are displayed:

<img src="https://yellow-negative-parrotfish-381.mypinata.cloud/ipfs/bafkreiaa5pibnbyylscupu333zu3cnebxisriz3zo6cjw3gplah32kt77e" alt="flow">

### Data Flow

1.) **Solar Power Generation**
Our cutting-edge solar hardware captures sunlight and converts it into energy that powers bitcoin miners. This energy production is securely tracked and sent to the Energy Oracle for verification in real time. <br>

2.) **Tokenization of Energy**
The Energy Oracle processes the energy data, ensuring its accuracy, and calculates the CSN token rewards based on how much energy you’ve produced. It’s a fair and transparent way to turn kilowatts into tokens. <br>

3.) **Powering Crypto Mining**
Your CSN tokens fuel our cryptocurrency mining operations, generating valuable digital assets. This process also produces heat, a byproduct we refuse to let go to waste and implement into our furnace system. <br>

4.) **Repurposing Waste Heat**
We capture the heat from mining and put it to good use, whether it’s heating buildings and houses, warming water, or other practical applications. Efficiency is the name of the game. <br>

5.) **Earning Rewards**
Participants like you earn CSN tokens based on the energy you contribute. The more clean energy you generate, the more rewards you take home, motivating sustainability every step of the way. <br>

6.) **Governance and Participation**
Holding CSN tokens gives you a say in the project’s future. Vote on protocol upgrades and improvements, ensuring CryptoSun evolves with the community’s vision at its core. <br>

## Deployed Contracts

The CryptoSun Program Library Program IDs:

| Program | Description | Program ID |
|---------|-------------|------------|
| CSN Token | Core token implementation with energy attributes | `csn1...TBA` |
| Energy Oracle | Connects real-world energy data to the blockchain | `eorg...TBA` |
| Staking | CSN token staking and rewards distribution | `stak...TBA` |
| Mining Registry | Registers and tracks mining hardware | `minr...TBA` |
| Energy Market | P2P marketplace for tokenized energy | `emkt...TBA` |
| Governance | On-chain voting and proposal system | `govn...TBA` |
| Heat Recovery | Tracks and rewards waste heat utilization | `heat...TBA` |

## Interact with the Codebase

### Prerequisites

- [Solana Tool Suite](https://docs.solana.com/cli/install-solana-cli-tools) v1.14.0 or later
- [Anchor](https://project-serum.github.io/anchor/getting-started/installation.html) v0.28.0 or later
- [Node.js](https://nodejs.org/) v16 or later
- [Yarn](https://yarnpkg.com/) v3 or later
- [Rust](https://www.rust-lang.org/tools/install) v1.68.0 or later

Welcome to the CryptoSun project! This section guides you through interacting with our Python-based codebase, from cloning the repository to contributing your own changes.

### Cloning the Repository
To get started, clone the repository:

    git clone https://github.com/Absolute-Solar/CryptoSun-Program-Library.git

### Setting Up the Environment

1.) Install Python: Ensure you have Python 3.8 or higher installed.<br>
2.) Create a Virtual Environment: 
    
    python -m venv venv
    source venv/bin/activate  # On Windows, use `venv\Scripts\activate`

3.) Install Dependencies: 

    pip install -r requirements.txt

4.) Configure Environment Variables: 

**Copy .env.example to .env and fill in values (e.g., API keys).**


### Running the Code

1.) Navigate to the Project Directory: 

    cd cryptosun


2.) Run the Application: 

    python main.py


3. Access the Application: 
Open your browser and go to http://localhost:5000.


### Testing
Run the tests to ensure your changes work:

    pytest

Contributing
We welcome contributions! Here’s how:<br>

1.) Fork the Repository: Click "Fork" on the repository page. <br>
2.) Create a Branch: 

    git checkout -b feature/your-feature-name


3.) Commit Your Changes: 

    git commit -m "Add your descriptive message"


4.) Push to Your Fork: 

    git push origin feature/your-feature-name


5.) Submit a Pull Request: Go to the original repository and click "New Pull Request."

See CONTRIBUTING.md for more details.

### Troubleshooting

Issue: ModuleNotFoundError: No module named 'cryptosun' <br>
Solution: Activate the virtual environment and install dependencies. <br>

Issue: "Port already in use" <br>
Solution: Change the port or stop the conflicting process. <br>


### Getting Help
Questions or issues?  

**Check the FAQ** <br>
**Search the issue tracker** <br>
**Join our Discord server**


## Absolute Solar DePIN Packages

CryptoSun provides a variety of hardware packages tailored for participants to join the network, contribute solar energy, and earn CSN tokens. Whether you're an individual exploring solar mining or a commercial operator aiming to scale, these packages offer flexible options to suit your needs. An optional add-on is also available to boost efficiency and rewards.

| Package | Description | Energy Production | Reward Potential | Additional Benefits |
|---------|-------------|-------------------|------------------|---------------------|
| Solar Miner Basic | Entry-level kit with a 1kW solar panel and basic mining rig | ~4 kWh/day | ~10 CSN/day | Perfect for beginners dipping into solar mining |
| Solar Miner Pro | Mid-tier kit featuring a 3kW solar panel and enhanced mining rig | ~12 kWh/day | ~35 CSN/day | Great balance of output and rewards |
| Solar Farm Node | Commercial-grade setup with a 10kW+ solar array and high-performance mining rig | ~40+ kWh/day | ~120+ CSN/day | Scalable for large-scale operations |
| Heat Recovery Add-on | Optional upgrade that captures and repurposes waste heat from mining | N/A | +20% bonus | Enhances efficiency and sustainability for any package |

## Deployments

### Mainnet

| Program | Address | Version |
|---------|---------|---------|
| CSN Token | `cs1...TBA` | vT.B.A |
| Energy Oracle | `eorg...TBA` | vT.B.A |
| Staking | `stak...TBA` | vT.B.A |
| Mining Registry | `minr...TBA` | vT.B.A |
| Energy Market | `emkt...TBA` | vT.B.A |

### Devnet

| Program | Address | Version |
|---------|---------|---------|
| CSN Token | `mntSrBkaf8fP9MpmYFGtfF2J9gKGt9PVyLB4mJeSsEn` | v1.0.0 |
| Energy Oracle | `eorg...TBA` | vT.B.A |
| Staking | `stak...TBA` | vT.B.A |
| Mining Registry | `minr...TBA` | vT.B.A |
| Energy Market | `emkt...TBA` | vT.B.A |

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
