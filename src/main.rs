use clap::{Parser, Subcommand};

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

    match &cli.command {
        Some(Commands::List { all, local, remote }) => {
            println!("listing command");
            if *local {
                println!("with local flag");
            } else if *remote {
                println!("with remote flag");
            } else if *all {
                println!("with all flag");
            } else {
                println!("with no flag");
            }
        }
        None => {
            println!("no command given (help?)");
        }
    }
}
