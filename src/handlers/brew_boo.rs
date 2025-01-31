use crate::config::Config;
use crate::contracts::{BrewBooV2, BrewBooV3, BrewContract};
use crate::strategies::{LiquidityPoolStrategy, SimpleStrategy, Strategy};
use ethers::prelude::*;
use eyre::Result;
use secrecy::{ExposeSecret, SecretString};
use std::{convert::TryFrom, sync::Arc};

type SignerClient = Arc<SignerMiddleware<Provider<Http>, LocalWallet>>;

pub async fn brew(
    private_key: SecretString,
    provider_gateway: String,
    version: String,
) -> Result<()> {
    // Load configuration
    let config = Config::load()?;

    let provider = Provider::<Http>::try_from(&provider_gateway)?;
    let chain_id = provider.get_chainid().await?;

    let wallet = private_key
        .expose_secret()
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

async fn execute_strategy(
    pairs: &[LiquidityPoolStrategy],
    contract: BrewContract<'_>,
    client: &SignerClient,
) -> Result<()> {
    let gas_est = client.get_gas_price().await?;

    let token_a: Vec<Address> = pairs.iter().map(|p| p.token_a).collect();
    let token_b: Vec<Address> = pairs.iter().map(|p| p.token_b).collect();
    let amounts: Vec<U256> = pairs
        .iter()
        .map(|p| p.amount.unwrap_or(U256::zero()))
        .collect();

    match contract {
        BrewContract::V2(contract) => {
            println!("Executing with V2 contract...");
            let brew_receipt = contract
                .convert_multiple(token_a, token_b, amounts)
                .gas_price(gas_est.as_u32())
                .gas(1700000)
                .send()
                .await?
                .await?;
            println!("V2 Result {:#?}", brew_receipt);
        }
        BrewContract::V3(contract) => {
            println!("Executing with V3 contract...");
            let brew_receipt = contract
                .convert_multiple(token_a, token_b, amounts)
                .gas_price(gas_est.as_u32())
                .gas(1700000)
                .send()
                .await?
                .await?;
            println!("V3 Result {:#?}", brew_receipt);
        }
    }

    Ok(())
}

async fn brew_v2(
    contract: BrewBooV2<SignerMiddleware<Provider<Http>, LocalWallet>>,
    client: SignerClient,
) -> Result<()> {
    let strategy = SimpleStrategy::new();
    println!("Executing strategy: {}", strategy.name());
    println!("Description: {}", strategy.description());

    execute_strategy(&strategy.get_pairs(), BrewContract::V2(&contract), &client).await
}

async fn brew_v3(
    contract: BrewBooV3<SignerMiddleware<Provider<Http>, LocalWallet>>,
    client: SignerClient,
) -> Result<()> {
    let strategy = SimpleStrategy::new();
    println!("Executing strategy: {}", strategy.name());
    println!("Description: {}", strategy.description());

    execute_strategy(&strategy.get_pairs(), BrewContract::V3(&contract), &client).await
}
