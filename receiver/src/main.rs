
mod parser;
use crate::parser::get_blob;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const CHANNEL: &str = "#ubruntu";

    let mut client = tmi::Client::anonymous().await?;
    client.join(CHANNEL).await?;

    loop {
        let msg = client.recv().await?;
        match msg.as_typed()? {
            tmi::Message::Privmsg(msg) => {
                if let Some(_blob) = get_blob(&mut msg.text()) {
                    // TODO: make directory with msg.sender().name()
                    // and download the same way `sendme receive` does
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
