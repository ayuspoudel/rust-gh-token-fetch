use clap::Parser;
use dotenvy::dotenv;
use std::fs;
use std::process;

use crate::fetch_token::fetch_and_write_token;

mod fetch_token;
mod jwt;

#[derive(Parser, Debug)]
#[command(name = "gh-token", version, about = "GitHub App Token Fetcher")]
struct Args {
    /// GitHub App ID
    #[arg(long, env = "GITHUB_APP_ID")]
    app_id: u64,

    /// GitHub Installation ID
    #[arg(long, env = "GITHUB_INSTALLATION_ID")]
    installation_id: u64,

    /// GitHub App private key (PEM string)
    #[arg(long, env = "GITHUB_APP_PRIVATE_KEY", required_unless_present = "private_key_path")]
    private_key: Option<String>,

    /// Path to private key PEM file
    #[arg(long, required_unless_present = "private_key")]
    private_key_path: Option<String>,

    /// Output path for .tfvars JSON
    #[arg(long, default_value = "./github.auto.tfvars.json")]
    output: String,
}

fn main() {
    // Load .env file if available
    dotenv().ok();

    let args = Args::parse();

    // Resolve private key
    let private_key = match (&args.private_key, &args.private_key_path) {
        (Some(inline), _) => inline.clone(),
        (_, Some(path)) => fs::read_to_string(path)
            .unwrap_or_else(|_| {
                eprintln!("❌ Failed to read private key from: {}", path);
                process::exit(1);
            }),
        _ => {
            eprintln!(" You must provide either --private-key or --private-key-path");
            process::exit(1);
        }
    };

    if let Err(err) = fetch_and_write_token(
        args.app_id,
        args.installation_id,
        &private_key,
        &args.output,
    ) {
        eprintln!("Failed to generate token: {err}");
        process::exit(1);
    }

    println!("Token written successfully to: {}", args.output);
}
