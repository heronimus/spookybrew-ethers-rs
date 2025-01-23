use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub contracts: Contracts,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contracts {
    pub brewboo_v2: ContractConfig,
    pub brewboo_v3: ContractConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractConfig {
    pub address: String,
    pub abi_path: String,
}

impl Config {
    pub fn load() -> eyre::Result<Self> {
        let config_str = fs::read_to_string("config.json")?;
        let config: Config = serde_json::from_str(&config_str)?;
        Ok(config)
    }
}
