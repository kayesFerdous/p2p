use clap::Parser;
use gub::cli::{Cli, Commands, GossipCommand};
use gub::receive::receive_file;
use gub::send::send_file;

use anyhow::Result;
use iroh::{Endpoint, protocol::Router};
use iroh_blobs::net_protocol::Blobs;

#[tokio::main]
async fn main() -> Result<()> {
    // Create an endpoint, it allows creating and accepting
    // connections in the iroh p2p world
    let endpoint = Endpoint::builder().discovery_n0().bind().await?;
    // We initialize the Blobs protocol in-memory
    let blobs = Blobs::memory().build(&endpoint);

    // Now we build a router that accepts blobs connections & routes them
    // to the blobs protocol.
    let router = Router::builder(endpoint)
        .accept(iroh_blobs::ALPN, blobs.clone())
        .spawn()
        .await?;

    // We use a blobs client to interact with the blobs protocol we're running locally:
    let blobs_client = blobs.client();

    let args = Cli::parse();
    match args.command {
        Commands::Send { filename } => {
            send_file(filename, blobs_client, &router).await?;
        }
        Commands::Receive { ticket, filename } => {
            receive_file(&filename, &ticket, blobs_client).await?;
        }
        Commands::Gossip { name, command } => match command {
            GossipCommand::Open => {}
            GossipCommand::Join { ticket } => {}
        },
    }

    // Gracefully shut down the node
    println!("Shutting down.");
    router.shutdown().await?;

    Ok(())
}
