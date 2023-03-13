use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

use crate::{
    config::Config,
    local::list_local_repos,
    remote::github,
};

mod config;
mod local;
mod remote;


#[derive(Debug, Serialize, Deserialize)]
struct GithubOrg {
    id: i32,
    login: String,
    description: String,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// list known local and remote repos
    List {
        /// only list local repos
        #[arg(short, long)]
        local: bool,
        /// only list remote repos
        #[arg(short, long)]
        remote: bool,
        /// list local and remote (default behavior)
        #[arg(short, long)]
        all: bool,
    },
    /// status summary of all potential local auth info
    Auth {},
    /// list of users and orgs remote has access to
    Orgs {},
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let default_root_folder = "~/repos";

    let config = Config {
        // TODO read from config file and parse it to a Path early
        root_folder: shellexpand::full(default_root_folder).unwrap().to_string(),
    };

    match &cli.command {
        Some(Commands::List {
            all: _,
            local,
            remote,
        }) => {
            println!("listing command");
            if *local {
                let local_repos = list_local_repos(&config).unwrap();
                println!("with local flag");
                eprintln!("DEBUGPRINT[1]: main.rs:43: local_repos={:?}", local_repos);
            } else if *remote {
                println!("with remote flag");
            } else {
                println!("with no flag");
            }
        }
        Some(Commands::Auth { .. }) => {
            println!("\nSummary of auth infomation:");
            let github_token_string = match github::get_token() {
                Ok(mut token) => {
                    token.replace_range(8.., &"*".repeat(token.len() - 8));
                    format!("☑ {}", token)
                }
                Err(..) => format!("☒ could not get github token from gh cli"),
            };
            println!("  github token: {}", github_token_string)
        }
        Some(Commands::Orgs { .. }) => {
            if let Ok(token) = github::get_token() {
                match html_test(&token).await {
                    Ok(res) => {
                        eprintln!("DEBUGPRINT[1]: main.rs:79: res={:?}", res);
                    }
                    Err(err) => {
                        eprintln!("DEBUGPRINT[4]: main.rs:82: err={:?}", err);
                    }
                };
            };
        }
        None => {
            println!("no command given (help?)");
        }
    };
}

async fn html_test(token: &str) -> Result<Vec<GithubOrg>, reqwest::Error> {
    let client = reqwest::Client::new();
    let req = client
        .get("https://api.github.com/user/orgs")
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("Authorization", format!("Bearer {token}"))
        .header("User-agent", "repos-cli/v0.1.0");

    let res = req.send().await?;

    let res: Vec<GithubOrg> = res.json().await?;

    Ok(res)
}
