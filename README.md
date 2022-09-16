# spookybrew-ethers-rs
Rust implementation of ethers-rs to execute [Spookyswap](https://spooky.fi/#/) spookybrew contracts.

## WIP [Work In Progress]

To build the current state of the program:
- Change the wallet private keys on `src/handlers/brew_boo.rs` line 21
- Set the array parameter of the token address to change the LP Pairs you want to brew (Default Pair is FTM/USDC)
- Build command: `cargo build`
- Execute the generated binary: `./target/debug/spookybrew_simple`
