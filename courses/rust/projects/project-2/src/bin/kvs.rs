use std::env::current_dir;
use std::process::exit;

use clap::{Parser, Subcommand};

use kvs::{KvStore, KvsError};

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

fn main() -> kvs::Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Set { key, value } => {
            let mut store = KvStore::open(current_dir()?)?;
            store.set(key, value)?;
        }
        Commands::Get { key } => {
            let mut store = KvStore::open(current_dir()?)?;
            if let Some(value) = store.get(key)? {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
        }
        Commands::Rm { key } => {
            let mut store = KvStore::open(current_dir()?)?;
            match store.remove(key) {
                Ok(()) => {}
                Err(KvsError::KeyNotFound) => {
                    println!("Key not found");
                    exit(1);
                }
                Err(e) => return Err(e),
            }
        }
    }

    Ok(())
}
