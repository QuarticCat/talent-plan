use std::process::exit;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set the value of a string key to a string
    Set {
        /// A string key
        #[clap(value_parser)]
        key: String,

        /// The string value of the key
        #[clap(value_parser)]
        value: String,
    },

    /// Get the string value of a given string key
    Get {
        /// A string key
        #[clap(value_parser)]
        key: String,
    },

    /// Remove a given key
    Rm {
        /// A string key
        #[clap(value_parser)]
        key: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Set { .. } => {
            eprintln!("unimplemented");
            exit(1);
        }
        Commands::Get { .. } => {
            eprintln!("unimplemented");
            exit(1);
        }
        Commands::Rm { .. } => {
            eprintln!("unimplemented");
            exit(1);
        }
    }
}
