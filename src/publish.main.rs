mod mq;
mod types;

use lapin::{options::BasicPublishOptions, BasicProperties};
use mq::{ensure_exchange, ensure_queue, get_connection, EXCHANGE};
use tokio;
use types::{BoardgameSite, DiscoverMessage};

#[tokio::main]
async fn main() -> Result<(), lapin::Error> {
    let conn = get_connection().await?;
    let channel = conn.create_channel().await?;
    ensure_exchange(&channel).await?;
    ensure_queue(&channel, &BoardgameSite::Spelonk).await?;
    ensure_queue(&channel, &BoardgameSite::ThePlayground).await?;

    let message = DiscoverMessage {
        msg_type: "discover".to_string(),
        foo: "bar".to_string(),
    };
    let payload = serde_json::to_vec(&message).unwrap();

    // let payload = br##"{"type":"discover", "foo": "bar"}"##;
    channel
        .basic_publish(
            EXCHANGE,
            &BoardgameSite::Spelonk.to_string(),
            BasicPublishOptions::default(),
            &payload,
            BasicProperties::default(),
        )
        .await?
        .await?;

    println!("Message published!");

    Ok(())
}
