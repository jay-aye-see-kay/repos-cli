use clap::{Parser, Subcommand};

use crate::{config::Config, local::list_local_repos, remote::github};

mod config;
mod local;
mod remote;

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
                println!("with --all or no flag");
            }
        }

        Some(Commands::Auth { .. }) => {
            println!("\nSummary of auth infomation:");
            match github::get_token() {
                Ok(mut token) => {
                    token.replace_range(8.., &"*".repeat(token.len() - 8));
                    println!("  - ☑ github token: {}", token);
                }
                Err(..) => println!("  - ☒ could not get github token from gh cli"),
            };
        }

        Some(Commands::Orgs { .. }) => {
            match github::get_token() {
                Ok(token) => {
                    match github::fetch_orgs(&token).await {
                        Ok(res) => println!("success with orgs={:?}", res),
                        Err(err) => println!("failed with err={:?}", err),
                    };
                }
                Err(..) => {
                    println!("no valid github token")
                }
            };
        }
        None => {
            println!("no command given (help?)");
        }
    };
}

