// fn main() {

// look for blobs

// are blobs always the same length?

// when a blob is found, download it (to ~/Downloads)

// when I receive something, put it in a folder named after who sent the msg

use crate::parser::is_blob;

mod parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const CHANNEL: &str = "#ubruntu";

    let mut client = tmi::Client::anonymous().await?;
    client.join(CHANNEL).await?;

    loop {
        let msg = client.recv().await?;
        match msg.as_typed()? {
            tmi::Message::Privmsg(msg) => {
                // look for blobs
                let mut txt = msg.text();
                if is_blob(&mut txt) {
                    println!("🐻 detected");
                }
                println!("{}: {}", msg.sender().name(), msg.text());
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
// }
