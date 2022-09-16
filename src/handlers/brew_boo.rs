use ethers::{prelude::*};
use eyre::Result;
use std::{convert::TryFrom, sync::Arc};

// Generate the type-safe contract bindings by providing the ABI definition
abigen!(
    BrewBooV3,
    "./src/abi/brewboo_v3.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

pub async fn brew() -> Result<()> {

    // connect to the network
    let provider = Provider::<Http>::try_from(
        "https://fantom-rpc.gateway.pokt.network"
    )?;
    let chain_id = provider.get_chainid().await?;

    // instantiate the wallet
    let wallet = "xxxx-your-wallet-private-key-xxxx"
        .parse::<LocalWallet>()?
        .with_chain_id(chain_id.as_u64());

    // instantiate the client with the wallet
    let client = Arc::new(SignerMiddleware::new(provider, wallet));

    // Initiate brewBoo
    let brewboo_v3_addr = "0x3B3fdC40582a957206Aed119842F2313DE9eE21b".parse::<Address>()?;
    let brewboo_v3_contract = BrewBooV3::new(brewboo_v3_addr, client.clone());

    // Send transaction
    let gas_est = client.get_gas_price().await?;
    let account = client.address();
    let balance = client.get_balance(account, None).await?;
    println!("Gas Price: {:?}", gas_est);
    println!("Account: {:?}", account);
    println!("Balance: {:?}", balance);

    let brew_receipt = brewboo_v3_contract.convert_multiple(
        vec!["0x21be370D5312f44cB42ce377BC9b8a0cEF1A4C83".parse::<Address>()?],
        vec!["0x04068DA6C83AFCFA0e13ba15A6696662335D5B75".parse::<Address>()?],
        Vec::new()
    ).gas_price(gas_est.as_u32()).gas(1700000).send().await?.await?;

    println!("Result {:#?}", brew_receipt);

    Ok(())
}
