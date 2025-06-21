use std::fs;
use std::path::Path;

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
            init_new_repo();
        }
        None => {
            println!("No command specified. Use --help for more information.");
        }
    }
}

fn create_dir(dir_path: String) {
    let create_result = fs::create_dir(&dir_path);

    match create_result {
        Ok(_) => {
            // Directory created successfully or already existed
            if Path::new(&dir_path).is_dir() {
                println!("'{}' Generated...", dir_path);
            }
        }
        Err(e) => {
            // Handle the error, e.g., print it
            eprintln!("Error creating directory '{}': {}", dir_path, e);
        }
    }
}

fn init_new_repo() {
    create_dir("./.sgit".to_string());
}
