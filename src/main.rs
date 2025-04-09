use clap::Parser;
use gub::cli::{Cli, Commands, GossipCommand};
use gub::gossip_file::{join_gossip_room, open_gossip_room};
use gub::receive::receive_file;
use gub::send::send_file;

use anyhow::Result;
use iroh::{Endpoint, protocol::Router};
use iroh_blobs::net_protocol::Blobs;
use iroh_gossip::net::Gossip;

#[tokio::main]
async fn main() -> Result<()> {
    // Create an endpoint, it allows creating and accepting
    // connections in the iroh p2p world
    let endpoint = Endpoint::builder().discovery_n0().bind().await?;
    // We initialize the Blobs protocol in-memory
    let blobs = Blobs::memory().build(&endpoint);
    let gossip = Gossip::builder().spawn(endpoint.clone()).await?;

    // Now we build a router that accepts blobs connections & routes them
    // to the blobs protocol.
    let router = Router::builder(endpoint)
        .accept(iroh_blobs::ALPN, blobs.clone())
        .accept(iroh_gossip::ALPN, gossip.clone())
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
            GossipCommand::Open => open_gossip_room(name).await?,
            GossipCommand::Join { ticket } => join_gossip_room(ticket, name).await?,
        },
    }

    // Gracefully shut down the node
    println!("Shutting down.");
    router.shutdown().await?;

    Ok(())
}
