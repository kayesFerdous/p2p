use anyhow::{Ok, Result};
use iroh_blobs::store::{ExportFormat, ExportMode};
use iroh_blobs::{rpc::client::blobs::MemClient, ticket::BlobTicket};
use std::path::PathBuf;

pub async fn receive_file(filename: &str, ticket: &str, blobs_client: &MemClient) -> Result<()> {
    let filename: PathBuf = filename.parse()?;
    let abs_path = std::path::absolute(filename)?;
    let ticket: BlobTicket = ticket.parse()?;

    println!("Starting download.");

    blobs_client
        .download(ticket.hash(), ticket.node_addr().clone())
        .await?
        .finish()
        .await?;

    println!("Finished download.");
    println!("Copying to destination.");

    blobs_client
        .export(
            ticket.hash(),
            abs_path,
            ExportFormat::Blob,
            ExportMode::Copy,
        )
        .await?
        .finish()
        .await?;

    println!("Finished copying.");
    Ok(())
}
