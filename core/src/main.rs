mod notify_me;

use clap::{Parser, Subcommand};
use notify_me::notify_me;

#[derive(Debug, Parser)]
#[command(
    name = "notify",
    about = "See your most important changes at a glance",
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(
        name = "me",
        about = "Read all notifications",
    )]
    NotifyMe {},
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::NotifyMe {} => {
            notify_me();
        },

    }
}
