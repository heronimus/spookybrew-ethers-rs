use ethers::prelude::*;

#[derive(Debug, Clone)]
pub struct LiquidityPoolStrategy {
    pub token_a: Address,
    pub token_b: Address,
    pub amount: Option<U256>,
}

pub trait Strategy {
    fn get_pairs(&self) -> Vec<LiquidityPoolStrategy>;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}
