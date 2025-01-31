use ethers::prelude::*;

// Generate the type-safe contract bindings by providing the ABI definition
abigen!(
    BrewBooV2,
    r"src/abi/brewboo_v2.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

abigen!(
    BrewBooV3,
    r"src/abi/brewboo_v3.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

#[derive(Debug)]
pub enum BrewContract<'a> {
    V2(&'a BrewBooV2<SignerMiddleware<Provider<Http>, LocalWallet>>),
    V3(&'a BrewBooV3<SignerMiddleware<Provider<Http>, LocalWallet>>),
}
