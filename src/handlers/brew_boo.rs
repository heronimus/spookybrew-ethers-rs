use crate::config::Config;
use crate::contracts::BrewBooV2;
use ethers::prelude::*;
use eyre::Result;
use std::{convert::TryFrom, sync::Arc};

pub async fn brew(private_key: String, provider_gateway: String) -> Result<()> {
    // Load configuration
    let config = Config::load()?;

    // Use provider_gateway instead of hardcoded URL
    let provider = Provider::<Http>::try_from(&provider_gateway)?;
    let chain_id = provider.get_chainid().await?;

    // Use private_key instead of hardcoded value
    let wallet = private_key
        .parse::<LocalWallet>()?
        .with_chain_id(chain_id.as_u64());

    // instantiate the client with the wallet
    let client = Arc::new(SignerMiddleware::new(provider, wallet));

    // Get contract address from config
    let brewboo_v3_addr = config.contracts.brewboo_v3.address.parse::<Address>()?;
    let brewboo_v3_contract = BrewBooV2::new(brewboo_v3_addr, client.clone());

    // Send transaction
    let gas_est = client.get_gas_price().await?;
    let account = client.address();
    let balance = client.get_balance(account, None).await?;
    println!("Gas Price: {:?}", gas_est);
    println!("Account: {:?}", account);
    println!("Balance: {:?}", balance);

    let brew_receipt = brewboo_v3_contract
        .convert_multiple(
            vec!["0x039e2fB66102314Ce7b64Ce5Ce3E5183bc94aD38".parse::<Address>()?], // wrapped S
            vec!["0x29219dd400f2Bf60E5a23d13Be72B486D4038894".parse::<Address>()?], // USDC.e
            Vec::new(),
        )
        .gas_price(gas_est.as_u32())
        .gas(1700000)
        .send()
        .await?
        .await?;

    println!("Result {:#?}", brew_receipt);

    Ok(())
}
