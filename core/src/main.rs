extern crate core;

mod notify_me;
mod watch;
mod confirm;
mod git;
mod store;

use std::path::PathBuf;
use clap::{Parser, Subcommand};
use notify_me::notify_me;
use crate::git::GitRepo;
use crate::store::{find_and_load_store, load_store, Store};

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

    /// Confirm a file change so that it doesn't appear until next update
    #[command(about)]
    Confirm {
        /// Path of the file to track
        file: String,
    }
}

pub struct Context {
    repo: GitRepo,
    store: Store,
}

fn main() {
    let args = Cli::parse();

    let repo = GitRepo::new();
    let store = find_and_load_store();
    let context = Context {repo, store};

    match args.command {
        Commands::NotifyMe {} => {
            notify_me();
        },
        Commands::Watch {file} => {
            watch::watch(file);
        },
        Commands::Confirm {file} => {
            confirm::confirm(context, file);
        },
    }
}
