use spookybrew_ethers_rs::handlers;
use clap::Parser;

#[derive(Parser)] // requires `derive` feature
#[command(name = "SpookyBrew")]
#[command(bin_name = "spookybrew")]
enum SpookyBrewCli {
    Brew(BrewArgs),
}

#[derive(clap::Args)]
#[command(author, version, about, long_about = None)]
struct BrewArgs {
   #[arg(short, long)]
   private_key: String,

   #[arg(short, long)]
   provider_gateway: String,
}

#[tokio::main]
async fn main() {
    let SpookyBrewCli::Brew(args) = SpookyBrewCli::parse();
    handlers::brew_boo::brew().await.unwrap();
}
