use super::types::{LiquidityPoolStrategy, Strategy};
use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct PairConfig {
    token_a: String,
    token_b: String,
    #[serde(default)]
    amount: Option<String>,
}

pub struct DynamicExternalPairStrategy {
    pairs: Vec<LiquidityPoolStrategy>,
    config_path: String,
}

impl DynamicExternalPairStrategy {
    pub fn new(config_path: &str) -> eyre::Result<Self> {
        let pairs = Self::load_pairs_from_file(config_path)?;
        Ok(Self {
            pairs,
            config_path: config_path.to_string(),
        })
    }

    fn load_pairs_from_file(path: &str) -> eyre::Result<Vec<LiquidityPoolStrategy>> {
        if !Path::new(path).exists() {
            return Err(eyre::eyre!("Pairs configuration file not found"));
        }

        let config_str = fs::read_to_string(path)?;
        let pair_configs: Vec<PairConfig> = serde_json::from_str(&config_str)?;

        let pairs: Result<Vec<LiquidityPoolStrategy>, _> = pair_configs
            .into_iter()
            .map(|config| {
                let token_a = config
                    .token_a
                    .parse()
                    .map_err(|_| eyre::eyre!("Invalid token A address"))?;
                let token_b = config
                    .token_b
                    .parse()
                    .map_err(|_| eyre::eyre!("Invalid token B address"))?;
                let amount = match config.amount {
                    Some(amt_str) => Some(
                        U256::from_dec_str(&amt_str)
                            .map_err(|_| eyre::eyre!("Invalid amount format"))?,
                    ),
                    None => None,
                };

                Ok(LiquidityPoolStrategy {
                    token_a,
                    token_b,
                    amount,
                })
            })
            .collect();

        pairs
    }

    pub fn reload_pairs(&mut self) -> eyre::Result<()> {
        self.pairs = Self::load_pairs_from_file(&self.config_path)?;
        Ok(())
    }
}

impl Strategy for DynamicExternalPairStrategy {
    fn get_pairs(&self) -> Vec<LiquidityPoolStrategy> {
        self.pairs.clone()
    }

    fn name(&self) -> &str {
        "Dynamic External Pair Strategy"
    }

    fn description(&self) -> &str {
        "A strategy that loads LP token pairs from an external JSON configuration file"
    }
}
