use std::{env, path::PathBuf};

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::daemon::{start_daemon, status_daemon, stop_daemon};

mod daemon;
mod highlighter;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Start the highlighter daemon
    Start,

    /// Stop the highlighter daemon
    Stop,

    /// Check whether the highlighter daemon is running
    Status,
}

fn main() -> Result<()> {
    let home = env::var("HOME").expect("$HOME not set");
    let data_dir = PathBuf::from(home).join(".local/share/zsh-patina");

    let args = Args::parse();

    match args.command {
        Command::Start => start_daemon(&data_dir),
        Command::Stop => stop_daemon(&data_dir),
        Command::Status => status_daemon(&data_dir),
    }
}
