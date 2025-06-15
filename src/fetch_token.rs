use std::fs;
use std::path::Path;

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::jwt::generate_jwt;

#[derive(Debug, Deserialize, Serialize)]
struct InstallationTokenResponse {
    token: String,
    expires_at: String,
}

/// Fetches a GitHub App installation token and writes it to a .tfvars file.
pub fn fetch_and_write_token(
    app_id: u64,
    installation_id: u64,
    private_key: &str,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating JWT...");
    let jwt = generate_jwt(app_id, private_key)?;

    let url = format!(
        "https://api.github.com/app/installations/{}/access_tokens",
        installation_id
    );
    println!("Requesting token from: {}", url);

    let client = Client::new();
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", jwt))
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "rust-token-fetch")
        .send()?
        .error_for_status()?;

    let token_res: InstallationTokenResponse = response.json()?;

    let output_dir = Path::new(output_path)
        .parent()
        .ok_or("Invalid output path")?;

    if !output_dir.exists() {
        println!("Creating output directory: {}", output_dir.display());
        fs::create_dir_all(output_dir)?;
    }

    let tfvars_content = json!({ "github_token": token_res.token });
    fs::write(output_path, tfvars_content.to_string())?;

    println!(" Token written to: {}", output_path);
    Ok(())
}
