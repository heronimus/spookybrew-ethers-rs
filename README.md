# SpookyBrew Ethers-rs

A Rust implementation using ethers-rs to interact with [SpookySwap](https://spooky.fi/#/) Brew contracts (currently migrated to Sonic). This tool allows you to automate the brewing process of BOO tokens from LP pairs.

## ⚠️ Disclaimer

This program serves as a basic template/reference implementation. Users should modify and optimize the code according to their specific brewing strategies. The default implementation is not necessarily optimal for all use cases.

## 🔧 Prerequisites

- Rust toolchain (1.56.0 or later)
- Valid wallet private key
- Access to a Sonic network RPC endpoint
- `jq` command tools
```bash
# For Ubuntu/Debian
sudo apt-get install jq
# For MacOS
brew install jq
```

## 🚀 Quick Start

1. Clone the repository:
```bash
git clone https://github.com/heronimus/spookybrew-ethers-rs.git
cd spookybrew-ethers-rs
```

2. Configure your environment:
   - Update `config.json` with appropriate contract addresses (current default is valid SpookyBrewV2 contract on Sonic)
   - Choose or implement your brewing strategy (see Strategies section below)

3. Build the project:
```bash
make build
```

4. Run the program:
```bash
./target/release/spookybrew_simple -k YOUR_PRIVATE_KEY_FILE_PATH -p YOUR_RPC_ENDPOINT -v v2
```

## 🛠️ Configuration

### Contract Configuration
Update `config.json` to match your target contract addresses:

```json
{
  "contracts": {
    "brewboo_v2": {
      "address": "0xc3815bF058fB94243Ebc6c559dfc59ceaEeF00eA",
      "abi_path": "src/abi/brewboo_v2.json"
    },
    "brewboo_v3": {
      "address": "0x79710d58c3600401fe21e799ff97f37100c8b179",
      "abi_path": "src/abi/brewboo_v3.json"
    }
  }
}
```

## 📝 Strategies

### Using Existing Strategies
The project comes with a `SimpleStrategy` that handles the wS/USDC.e pair by default.

### Implementing New Strategies
To create a new strategy:

1. Create a new file in `src/strategies/` (e.g., `custom_strategy.rs`)
2. Implement the `Strategy` trait:

```rust
use super::types::{Strategy, LiquidityPoolStrategy};
use ethers::prelude::*;

pub struct CustomStrategy {
    pairs: Vec<LiquidityPoolStrategy>,
}

impl CustomStrategy {
    pub fn new() -> Self {
        // Define your LP pairs here
        let pairs = vec![
            LiquidityPoolStrategy {
                token_a: "TOKEN_A_ADDRESS".parse().expect("Invalid token A address"),
                token_b: "TOKEN_B_ADDRESS".parse().expect("Invalid token B address"),
                amount: None, // Or Some(amount) for specific amounts
            },
            // Extend pairs with additional entries or populate from external data sources like price APIs
        ];

        Self { pairs }
    }
}

impl Strategy for CustomStrategy {
    fn get_pairs(&self) -> Vec<StrategyPair> {
        self.pairs.clone()
    }

    fn name(&self) -> &str {
        "Custom Strategy Name"
    }

    fn description(&self) -> &str {
        "Description of your strategy"
    }
}
```

3. Register your strategy in `src/strategies/mod.rs`:
```rust
mod custom_strategy;
pub use custom_strategy::CustomStrategy;
```

4. Use your strategy in the brew handler:
```rust
async fn brew_v2(
    contract: BrewBooV2<SignerMiddleware<Provider<Http>, LocalWallet>>,
    client: SignerClient,
) -> Result<()> {
    let strategy = CustomStrategy::new();
    // ... rest of the implementation
}
```

## 🔍 Optimization Opportunities

This template can be enhanced in several ways:

1. **Gas Optimization**: Implement dynamic gas price calculation
2. **Strategy Enhancements**:
   - Add strategy configuration via config files
   - Implement timing-based strategies
   - Create sophisticated routing strategies
3. **Monitoring**: Add logging and monitoring capabilities
4. **Testing**: Add comprehensive tests for strategies

## 🤝 Contributing

Contributions are welcome! Please feel free to submit pull requests or create issues for bugs and feature requests.

When contributing new strategies:
1. Ensure your strategy is well-documented
2. Include any specific configuration requirements
3. Add tests for your strategy
4. Update the README with strategy details if needed

## ⚠️ Risk Warning

- This is experimental software and comes with no warranties or guarantees
- Always test with small amounts first
- Verify all transactions before signing
- Keep your private keys secure

## 📚 Resources

- [SpookySwap Documentation](https://v3.docs.spooky.fi/)
- [Ethers-rs Documentation](https://docs.rs/ethers/latest/ethers/)
