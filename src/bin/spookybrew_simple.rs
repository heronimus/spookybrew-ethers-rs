use spookybrew_ethers_rs::handlers;

#[tokio::main]
async fn main() {
    handlers::brew_boo::brew().await.unwrap();
}
