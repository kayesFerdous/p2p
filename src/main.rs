use clap::Parser;
use zerogate::cli::{Cli, Commands, GossipCommand};
use zerogate::gossip_file::{join_gossip_room, open_gossip_room};
use zerogate::receive::receive_file;
use zerogate::send::send_file;

use anyhow::Result;
use iroh::{Endpoint, protocol::Router};
use iroh_blobs::net_protocol::Blobs;
use iroh_gossip::net::Gossip;

#[tokio::main]
async fn main() -> Result<()> {
    let endpoint = Endpoint::builder().discovery_n0().bind().await?;
    let blobs = Blobs::memory().build(&endpoint);
    let gossip = Gossip::builder().spawn(endpoint.clone()).await?;

    let router = Router::builder(endpoint)
        .accept(iroh_blobs::ALPN, blobs.clone())
        .accept(iroh_gossip::ALPN, gossip.clone())
        .spawn()
        .await?;

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

    println!("Shutting down.");
    router.shutdown().await?;

    Ok(())
}
