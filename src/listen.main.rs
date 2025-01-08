mod mq;
mod types;

use std::{thread::sleep, time::Duration};

use lapin::options::BasicAckOptions;
use mq::{ensure_exchange, ensure_queue, get_connection};
use serde::{Deserialize, Serialize};
use tokio;
use types::{BoardgameSite, DiscoverMessage, Message};

#[tokio::main]
async fn main() -> Result<(), lapin::Error> {
    let site = BoardgameSite::Spelonk;
    let conn = get_connection().await?;
    let channel = conn.create_channel().await?;
    ensure_exchange(&channel).await?;
    ensure_queue(&channel, &site).await?;

    loop {
        let msg = channel
            .basic_get(
                &site.to_string(),
                lapin::options::BasicGetOptions::default(),
            )
            .await?;

        if let Some(msg) = msg {
            let message: Message =
                serde_json::from_slice(&msg.data).expect("Deserialization failed");
            let discover_message: DiscoverMessage =
                serde_json::from_slice(&msg.data).expect("Deserialization failed");

            println!("Message: {:?}", message);
            println!("Discover message: {:?}", discover_message);

            channel
                .basic_ack(msg.delivery_tag, BasicAckOptions::default())
                .await?;
        } else {
            sleep(Duration::from_secs(1));
        }
    }
}
