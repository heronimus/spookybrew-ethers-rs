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

pub struct StrategyDynamicExternalPair {
    pairs: Vec<LiquidityPoolStrategy>,
    config_path: String,
}

impl StrategyDynamicExternalPair {
    pub fn new(config_path: &str) -> Self {
        let pairs = Self::load_pairs_from_file(config_path);
        Self {
            pairs,
            config_path: config_path.to_string(),
        }
    }

    fn load_pairs_from_file(path: &str) -> Vec<LiquidityPoolStrategy> {
        if !Path::new(path).exists() {
            return vec![];
        }

        let config_str = match fs::read_to_string(path) {
            Ok(str) => str,
            Err(_) => return vec![],
        };

        let pair_configs: Vec<PairConfig> = match serde_json::from_str(&config_str) {
            Ok(configs) => configs,
            Err(_) => return vec![],
        };

        pair_configs
            .into_iter()
            .filter_map(|config| {
                let token_a = match config.token_a.parse() {
                    Ok(addr) => addr,
                    Err(_) => return None,
                };
                let token_b = match config.token_b.parse() {
                    Ok(addr) => addr,
                    Err(_) => return None,
                };
                let amount = match config.amount {
                    Some(amt_str) => match U256::from_dec_str(&amt_str) {
                        Ok(amt) => Some(amt),
                        Err(_) => return None,
                    },
                    None => None,
                };

                Some(LiquidityPoolStrategy {
                    token_a,
                    token_b,
                    amount,
                })
            })
            .collect()
    }

    pub fn reload_pairs(&mut self) -> eyre::Result<()> {
        self.pairs = Self::load_pairs_from_file(&self.config_path);
        Ok(())
    }
}

impl Strategy for StrategyDynamicExternalPair {
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
