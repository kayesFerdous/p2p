use std::path::PathBuf;

use crate::clipboard::clipboard;
use anyhow::Result;
use iroh::protocol::Router;
use iroh_blobs::rpc::client::blobs::{MemClient, WrapOption};
use iroh_blobs::ticket::BlobTicket;
use iroh_blobs::util::SetTagOption;

pub async fn send_file(filename: PathBuf, blobs_client: &MemClient, router: &Router) -> Result<()> {
    let abs_path = std::path::absolute(&filename)?;

    println!("Hashing file ...");

    let in_place = true;
    let blob = blobs_client
        .add_from_path(abs_path, in_place, SetTagOption::Auto, WrapOption::NoWrap)
        .await?
        .finish()
        .await?;

    let node_id = router.endpoint().node_id();
    let ticket = BlobTicket::new(node_id.into(), blob.hash, blob.format)?;

    clipboard(ticket.to_string());
    println!("Ticket copied to clipboard!");

    println!("File hashed. Fetch this file by running:");
    println!("zerogate receive -t {ticket} {}", filename.display());

    tokio::signal::ctrl_c().await?;

    Ok(())
}
