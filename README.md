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
   - Update `config.json` with appropriate contract addresses (current default is valid SpookyBrewV2 contract on Sonic).
   - Customize the token pairs in `handlers/brew_boo.rs` to match your preferred strategy (Note: Future updates will include functionality to import LP pairs from external sources)

3. Build the project:
```bash
make build
```

4. Run the program:
```bash
./target/release/spookybrew_simple -k YOUR_PRIVATE_KEY -p YOUR_RPC_ENDPOINT
```

## 🛠️ Configuration

### Token Pairs
The default implementation uses wS/USDC.e pair. To modify LP pairs, update the `convert_multiple` parameters in `handlers/brew_boo.rs` (Note: Future updates will include functionality to import LP pairs from external sources):

```rust
let brew_receipt = brewboo_v3_contract
    .convert_multiple(
        vec!["TOKEN_A_ADDRESS".parse::<Address>()?],
        vec!["TOKEN_B_ADDRESS".parse::<Address>()?],
        Vec::new(),
    )
```

### Contract Configuration
Update `config.json` to match your target contract addresses (current default is valid SpookyBrewV2 contract on Sonic):

```json
{
  "contracts": {
    "brewboo_v2": {
      "address": "0xc3815bF058fB94243Ebc6c559dfc59ceaEeF00eA",
      "abi_path": "src/abi/brewboo_v2.json"
    }
  }
}
```

## 🔍 Optimization Opportunities

This template can be enhanced in several ways:

1. **Gas Optimization**: Implement dynamic gas price calculation
2. **Multiple LP Pairs**: Add support for batch processing multiple LP pairs
4. **Monitoring**: Add logging and monitoring capabilities
5. **Strategy Implementation**:
   - Timing optimization for brewing
   - Price impact checks
   - Slippage protection
   - Custom routing logic

## 🤝 Contributing

Contributions are welcome! Please feel free to submit pull requests or create issues for bugs and feature requests.

## ⚠️ Risk Warning

- This is experimental software and comes with no warranties or guarantees
- Always test with small amounts first
- Verify all transactions before signing
- Keep your private keys secure

## 📚 Resources

- [SpookySwap Documentation](https://v3.docs.spooky.fi/)
- [Ethers-rs Documentation](https://docs.rs/ethers/latest/ethers/)
