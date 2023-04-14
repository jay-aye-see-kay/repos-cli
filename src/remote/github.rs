use std::{io, process::Command};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubOrg {
    id: i32,
    login: String,
    description: String,
}

pub fn get_token() -> io::Result<String> {
    let output = Command::new("gh").arg("auth").arg("token").output()?;

    if !output.status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, ""));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.trim().to_string())
}

pub async fn fetch_orgs(token: &str) -> Result<Vec<GithubOrg>, reqwest::Error> {
    let client = reqwest::Client::new();
    let req = client
        .get("https://api.github.com/user/orgs")
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("Authorization", format!("Bearer {token}"))
        .header("User-agent", "repos-cli/v0.1.0");
    req.send().await?.json().await
}
