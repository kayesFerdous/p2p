use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "gub",
    version = "1.0",
    author = "Kayes <kayesfardows@gmail.com>",
    about = "🚀 Send & receive files, or gossip in real-time",
    long_about = "✨ A modern CLI tool for file sharing and chatting between peers.\n\
                  Built with Rust for fast and reliable communication.",
    disable_help_subcommand = true,
    arg_required_else_help = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 📤 Send a file to another peer
    Send {
        /// Path to the file to be sent
        #[clap(short, long, value_name = "FILE")]
        filename: PathBuf,
    },

    /// 📥 Receive a file from another peer
    Receive {
        /// Ticket provided by the sender
        #[clap(short, long, value_name = "TICKET")]
        ticket: String,

        /// Desired name for the received file
        #[clap(short, long, value_name = "FILENAME")]
        filename: String,
    },

    /// 💬 Start gossiping 
    Gossip {
        /// Your display name in the gossip
        #[clap(short, long)]
        name: Option<String>,

        #[clap(subcommand)]
        command: GossipCommand,
    },
}

#[derive(Subcommand)]
pub enum GossipCommand {
    /// Open a new gossip
    Open,

    /// Join an existing gossip
    Join {
        /// Ticket used to join the gossip
        #[clap(short, long, value_name = "TICKET")]
        ticket: String,
    },
}
