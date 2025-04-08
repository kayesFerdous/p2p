use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Send {
        #[clap(short, long)]
        filename: PathBuf,
    },

    Receive {
        #[clap(short, long)]
        ticket: String,

        #[clap(short, long)]
        filename: String,
    },

    Gossip {
        #[clap(short, long)]
        name: Option<String>,

        #[clap(subcommand)]
        command: GossipCommand,
    },
}

#[derive(Subcommand)]
pub enum GossipCommand {
    Open,
    Join {
        #[clap(short, long)]
        ticket: String,
    },
}
