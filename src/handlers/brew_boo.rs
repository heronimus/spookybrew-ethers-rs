use crate::config::Config;
use crate::contracts::{BrewBooV2, BrewBooV3};
use ethers::prelude::*;
use eyre::Result;
use std::{convert::TryFrom, sync::Arc};

pub async fn brew(private_key: String, provider_gateway: String, version: String) -> Result<()> {
    // Load configuration
    let config = Config::load()?;

    let provider = Provider::<Http>::try_from(&provider_gateway)?;
    let chain_id = provider.get_chainid().await?;

    let wallet = private_key
        .parse::<LocalWallet>()?
        .with_chain_id(chain_id.as_u64());

    let client = Arc::new(SignerMiddleware::new(provider, wallet));

    match version.as_str() {
        "v2" => {
            let contract_addr = config.contracts.brewboo_v2.address.parse::<Address>()?;
            let contract = BrewBooV2::new(contract_addr, client.clone());
            brew_v2(contract, client).await
        }
        "v3" => {
            let contract_addr = config.contracts.brewboo_v3.address.parse::<Address>()?;
            let contract = BrewBooV3::new(contract_addr, client.clone());
            brew_v3(contract, client).await
        }
        _ => Err(eyre::eyre!("Unsupported version. Use 'v2' or 'v3'")),
    }
}

async fn brew_v2(
    contract: BrewBooV2<SignerMiddleware<Provider<Http>, LocalWallet>>,
    client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
) -> Result<()> {
    // Existing V2 implementation
    let gas_est = client.get_gas_price().await?;
    let account = client.address();
    let balance = client.get_balance(account, None).await?;
    println!("Gas Price: {:?}", gas_est);
    println!("Account: {:?}", account);
    println!("Balance: {:?}", balance);

    let brew_receipt = contract
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

async fn brew_v3(
    contract: BrewBooV3<SignerMiddleware<Provider<Http>, LocalWallet>>,
    client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
) -> Result<()> {
    // Implement V3-specific logic here
    let gas_est = client.get_gas_price().await?;
    let account = client.address();
    let balance = client.get_balance(account, None).await?;
    println!("Gas Price: {:?}", gas_est);
    println!("Account: {:?}", account);
    println!("Balance: {:?}", balance);

    let brew_receipt = contract
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
