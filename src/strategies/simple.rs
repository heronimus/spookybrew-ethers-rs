use super::types::{LiquidityPoolStrategy, Strategy};

pub struct SimpleStrategy {
    pairs: Vec<LiquidityPoolStrategy>,
}

impl SimpleStrategy {
    pub fn new() -> Self {
        // Default wS/USDC.e pair
        let pairs = vec![LiquidityPoolStrategy {
            token_a: "0x039e2fB66102314Ce7b64Ce5Ce3E5183bc94aD38"
                .parse()
                .expect("Invalid token A address"),
            token_b: "0x29219dd400f2Bf60E5a23d13Be72B486D4038894"
                .parse()
                .expect("Invalid token B address"),
            amount: None,
        }];

        Self { pairs }
    }
}

impl Strategy for SimpleStrategy {
    fn get_pairs(&self) -> Vec<LiquidityPoolStrategy> {
        self.pairs.clone()
    }

    fn name(&self) -> &str {
        "Simple wS/USDC.e Strategy"
    }

    fn description(&self) -> &str {
        "A simple strategy that converts wS/USDC.e LP tokens to BOO"
    }
}
