mod notify_me;
mod watch;
mod git;

use clap::{Parser, Subcommand};
use notify_me::notify_me;

/// See your most important changes at a glance
#[derive(Debug, Parser)]
#[command(name = "notify", about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(
        name = "me",
        about = "Read all notifications",
    )]
    NotifyMe {},

    #[command(
        about = "Watch a file"
    )]
    Watch {
        /// Path of the file to track
        file: String,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::NotifyMe {} => {
            notify_me();
        },
        Commands::Watch {file} => {
            watch::watch(file);
        }
    }
}
