use clap::{Parser, Subcommand};
use config::Config;

use crate::local::list_local_repos;

mod config;
mod local;

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
}

fn main() {
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
        None => {
            println!("no command given (help?)");
        }
    }
}
