use clap::Parser;
use secrecy::Secret;
use spookybrew_ethers_rs::handlers;
use std::{fs, process};

#[derive(Parser)]
#[command(name = "SpookyBrew")]
#[command(version = "1.0")]
#[command(about = "SpookyBrew - A tool for brewing BOO tokens")]
enum SpookyBrewCli {
    Brew(BrewArgs),
}

#[derive(clap::Args)]
#[command(author = "")]
#[command(version = "1.0")]
#[command(about = "Brew BOO tokens from LP tokens")]
struct BrewArgs {
    /// Path to file containing your private key
    #[arg(short = 'k', long)]
    private_key_path: String,

    /// RPC provider gateway URL
    #[arg(short = 'p', long)]
    provider_gateway: String,

    /// Contract version to use (v2 or v3)
    #[arg(short = 'v', long, default_value = "v2")]
    contract_version: String,
}

#[tokio::main]
async fn main() {
    // Initialize better error handling
    if let Err(e) = run().await {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

async fn run() -> eyre::Result<()> {
    println!("Starting SpookyBrew...");

    // Parse CLI arguments with error handling
    let SpookyBrewCli::Brew(args) = SpookyBrewCli::parse();

    // Validate private key format
    // Read and validate private key from file
    let private_key = read_private_key(&args.private_key_path)?;

    // Validate provider gateway URL
    if !validate_provider_url(&args.provider_gateway) {
        return Err(eyre::eyre!("Invalid provider gateway URL"));
    }

    // Validate contract version
    if !validate_contract_version(&args.contract_version) {
        return Err(eyre::eyre!(
            "Invalid contract version. Must be in format v2, v3, v4, etc."
        ));
    }

    println!("Connecting to network...");

    // Execute the brew operation
    match handlers::brew_boo::brew(private_key, args.provider_gateway, args.contract_version).await
    {
        Ok(_) => {
            println!("Brew operation completed successfully!");
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to execute brew operation");
            Err(e)
        }
    }
}

fn read_private_key(path: &str) -> eyre::Result<Secret<String>> {
    // Check if file exists and has proper permissions
    let metadata = fs::metadata(path)?;

    // On Unix-like systems, check file permissions
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = metadata.permissions().mode();
        // Check if file permissions are too open (should be 600 or more restrictive)
        if mode & 0o077 != 0 {
            return Err(eyre::eyre!(
                "Private key file permissions too open. Please set to 600 or more restrictive"
            ));
        }
    }

    // Read and trim the private key
    let private_key = fs::read_to_string(path)?.trim().to_string();

    // Validate the private key format
    if !validate_private_key(&private_key) {
        return Err(eyre::eyre!("Invalid private key format in file"));
    }

    Ok(Secret::new(private_key))
}

fn validate_private_key(key: &str) -> bool {
    // Basic validation: check if it's a valid hex string of correct length
    // A private key should be 64 characters long (32 bytes) when represented in hex
    if key.len() != 64 && key.len() != 66 {
        // 66 if prefixed with "0x"
        return false;
    }

    let key = if key.starts_with("0x") {
        &key[2..]
    } else {
        key
    };

    key.chars().all(|c| c.is_ascii_hexdigit())
}

fn validate_provider_url(url: &str) -> bool {
    // Basic URL validation
    url.starts_with("http://")
        || url.starts_with("https://")
        || url.starts_with("ws://")
        || url.starts_with("wss://")
}

fn validate_contract_version(version: &str) -> bool {
    version.starts_with('v') && version.len() > 1 && version[1..].parse::<u32>().is_ok()
}
