mod parser;
use crate::parser::get_blob;
use iroh_blobs::ticket::BlobTicket;
use shared::receive;
use std::fs;
use std::str::FromStr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const CHANNEL: &str = "#ubruntu";

    let mut client = tmi::Client::anonymous().await?;
    client.join(CHANNEL).await?;

    loop {
        let msg = client.recv().await?;
        match msg.as_typed()? {
            tmi::Message::Privmsg(msg) => {
                if let Some(blob) = get_blob(&mut msg.text()) {
                    let downloads_dir = dirs::home_dir()
                        .map(|home| home.join("Downloads").join(msg.sender().name().to_string()))
                        .ok_or_else(|| anyhow::anyhow!("Failed to determine home directory"))?;

                    if !downloads_dir.exists() {
                        fs::create_dir(&downloads_dir)?;
                        println!("Created directory: {:?}", downloads_dir);
                    } else {
                        println!("Directory already exists: {:?}", downloads_dir);
                    }
                    if let Ok(ticket) = BlobTicket::from_str(&blob) {
                        receive(ticket, &downloads_dir).await?;
                    } else {
                        println!("Received invalid blob: {:?}", blob);
                    }
                }
            }
            tmi::Message::Reconnect => {
                client.reconnect().await?;
                client.join(CHANNEL).await?;
            }
            tmi::Message::Ping(ping) => {
                client.pong(&ping).await?;
            }
            _ => {}
        }
    }
}
