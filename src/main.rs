use clap::{Parser, Subcommand};

/// A friendly CLI application
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Init,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init) => {
            InitNewRepo();
        }
        None => {
            println!("No command specified. Use --help for more information.");
        }
    }
}

fn InitNewRepo() {}
